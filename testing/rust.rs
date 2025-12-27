use std::{cell::RefCell, ops::Deref, rc::Rc};

use dioxus::{html::FileData, prelude::*};
use dioxus_sdk::sync::channel::{use_channel, use_listen_channel};
use epub_conversion::EpubView;
use err_trail::ErrContext;

use crate::{
    common::Pending,
    core::document::{Document, Epub, EpubLoadConfig},
    ui::{
        components::{
            ErrorPanel, LoadingPanel, epub_viewer::EpubViewer, viewer_scaffold::ViewerScaffold,
        },
        err_presenter::{GenericErrorRootOverlayHandler, PresentErr},
    },
    utils::{NeverEq, Reported, ReportedErr},
};

#[derive(Clone)]
pub enum DocumentEvent {
    NextPage,
    PrevPage,
}

const X: &str = "Hello";

#[derive(Props, PartialEq, Clone)]
pub struct DocumentViewerProps {
    id: ReadSignal<Option<String>>,
    search: ReadSignal<Option<serde_json::Value>>,
}

#[component]
pub fn DocumentViewer(props: DocumentViewerProps) -> Element {
    let DocumentViewerProps {
        id: document_id,
        search,
    } = props;
    let mut document: Signal<Option<Document>> = use_signal(|| None);
    let mut document_file_to_read: Signal<Option<FileData>> = use_signal(|| None);
    let loading_document: Resource<Reported<()>> = use_resource(move || async move {
        let bytes = if let Some(document_id) = &*document_id.read() {
            todo!("Load \n document")
        } else if let Some(file_data) = &*document_file_to_read.read() {
            let bytes = file_data.read_bytes().await.error(()).reported()?;
            bytes
        } else {
            return Ok(());
        };
        let load_config =
            serde_json::from_value::<EpubLoadConfig>(search.cloned().unwrap_or_default())
                .unwrap_or_default();
        let view = EpubView::new(bytes)
            .error(())
            .reported()
            // .present_err::<GenericErrorRootOverlayHandler>("Could not load epub")
            .map(|e| Rc::new(NeverEq::new(RefCell::new(e))))?;
        let epub = Epub::new(view, load_config);
        document.set(Some(Document::Epub(epub)));
        Ok(())
    });
    let document_event_channel = use_channel::<DocumentEvent>(1);
    use_context_provider(|| document_event_channel);

    match &*loading_document.value().read() {
        Pending::Some(result) => match result {
            Ok(_) => {}
            Err(_) => {
                return rsx! {
                    ViewerScaffold { ErrorPanel {} }
                };
            }
        },
        Pending::None => {
            return rsx! {
                ViewerScaffold { LoadingPanel {} }
            };
        }
    }
    let Some(doc) = document.cloned() else {
        let handle_file = move |e: Event<FormData>| async move {
            let files = e.files();
            if files.is_empty() {
                return;
            }
            debug_assert!(files.len() == 1);
            let file = files.into_iter().next().unwrap();
            document_file_to_read.set(Some(file));
        };
        return rsx! {
            div {
                div { "Load an epub to view it here." }
                input {
                    r#type: "file",
                    accept: ".epub",
                    multiple: false,
                    onchange: handle_file,
                }
            }
        };
    };
    match &doc {
        Document::Epub(epub) => rsx! {
            ViewerScaffold { doc: doc.clone(),
                EpubViewer { epub: epub.clone() }
            }
        },
        Document::Pdf => todo!(),
    }
}
