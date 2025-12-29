# Notes

Notes related to understanding the repo structure and inner workings of this project

## Structure

- `src/` contains the inner code for viewing the pdf and plumbing. This is what the semver is based on
- `web/` contains the code built on top of `src/` that has an actual working pdf viewer with features outside just rendering like annotations

## Developing

Gulp is a task runner/toolkit and running 
```bash
bunx gulp server
```
will launch the server to use the viewer

## `web/`

`viewer.html` contains the entry **point** for the viewer application. Each feature is associated with an id. e.g. `id="viewsManagerHeader"`. This ids are used by the script `