import { Relay } from "@mcmah309/epub-component/packages/window-relay";
import { EpubController } from "./controller-bridge";
import type { JsValue } from "../../common";
import { isDebug } from "../../macros" with { type: 'macro' };
import type { SetChapterPayload } from "../communication_types";

/**
 * Sets up connect with the epub in the iframe
 */
export async function setupEpubIframeConnection(iframeId: string): Promise<JsValue<EpubController>> {
    const iframe = document.getElementById(iframeId);
    if (!iframe) {
        throw new Error(`Iframe with ID '${iframeId}' not found`);
    }
    if (iframe instanceof HTMLIFrameElement === false) {
        throw new Error(`Element with ID '${iframeId}' is not an iframe`);
    }
    let getIframe = isDebug() ? () => {
        let currentIframe = document.getElementById(iframeId);
        if (iframe !== currentIframe) {
            throw new Error("Iframe has changed!");
        }
        return iframe;
    } : iframe;

    const relay = Relay.createToIframeRelay(getIframe, {
        // validateMessage: (event) => event.source === iframe.contentWindow,
    });

    async function nextPage() {
        await relay.invoke("nextPage");
    }

    async function prevPage() {
        await relay.invoke("prevPage");
    }

    async function setChapter(headHtml: string, bodyHtml: string) {
        await relay.invoke("setChapter", [headHtml, bodyHtml] as SetChapterPayload);
    }

    let controller = new EpubController(nextPage, prevPage, setChapter);

    relay.establishConnection();
    await relay.waitForRemoteReady();

    console.trace("Relay connection established!");

    return controller;
}

export class EpubController {
    _nextPage: () => Promise<void>;
    _prevPage: () => Promise<void>;
    _setChapter: (headHtml: string, bodyHtml: string) => Promise<void>;

    constructor(nextPage: () => Promise<void>, prevPage: () => Promise<void>, setChapter: (headHtml: string, bodyHtml: string) => Promise<void>) {
        this._nextPage = nextPage;
        this._prevPage = prevPage;
        this._setChapter = setChapter;
    }

    async setChapter(headHtml: string, bodyHtml: string): Promise<void> {
        return this._setChapter(headHtml, bodyHtml);
    }

    async nextPage(): Promise<void> {
        return this._nextPage();
    }

    async prevPage(): Promise<void> {
        return this._prevPage();
    }
}