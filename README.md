# Screen 13

[![Crates.io](https://img.shields.io/crates/v/screen-13.svg)](https://crates.io/crates/screen-13)
[![Docs.rs](https://docs.rs/screen-13/badge.svg)](https://docs.rs/screen-13)

Screen 13 is an easy-to-use 3D game engine in the spirit of QBasic.

## Overview

Games made using Screen 13 are built as regular executables using a design-time asset baking process. Screen 13 provides all asset-baking logic and aims to, but currently does not, provide wide support for texture formats, vertex formats, and other associated data. Baked assets are stored in `.pak` files.

Screen 13 is based on the [`gfx-rs`](https://github.com/gfx-rs/gfx) project, and as such targets native Vulkan, Metal, DirectX 12, OpenGL, WebGL, Android, and iOS targets, among others.

### Goals

Screen 13 aims to provide a simple to use, although opinionated, ecosystem of tools and code that enable very high performance portable graphics programs for developers using the Rust programming language.

_Single Threaded:_ Although some things can be shared amongst other threads, such as disk and network IO, the main graphics API of Screen 13 does not support multiple threads. This is a conscious decision to limit complexity while optimizing for the 98% of games that use a "main thread" methodology. I am open to changing this if the proposed API is easy to use and high performance.

## Asset Baking

Asset baking is the process of converting files from their native file formats into a runtime-ready format that is optimized for both speed and size. Currently Screen 13 uses a single file (or single HTTP/S endpoint) for all runtime assets. Assets are baked from `.toml` files which you can find examples of in the `examples/content` directory.

## Quick Start

Included is an example you might find helpful:

- `basic.rs` - Displays 'Hello, World!' on the screen. Please start here.

The example requires an associated asset `.pak` file in order to run, so you will need to run the example like so:

```bash
cargo run examples/content/basic.toml
cargo run --example basic
```

These commands do the following:

- Build the Screen 13 engine (_runtime_) and executable code (_design-time_)
- Bake the assets from `basic.toml` into `basic.pak`
- Runs the `basic` example (Close window to exit)

See the example code for more information, including a helpful [getting started guide](examples/README.md).

## Roadmap/Status/Notes

This engine is very young and is likely to change as development continues.

- Requires [Rust](https://www.rust-lang.org/) 1.45 _or later_
- _Design-time_ Asset Baking:
  - Animation - **Rotations only**, no scaling, morph targets, or root motion
  - Bitmaps
    - Wide format support: .png, .jpg, _etc..._
    - 1-4 channel support
    - Unpacked using GPU compute hardware at runtime
  - Blobs (raw file byte vectors)
  - Language file (locale to key/value dictionary)
  - Material file
    - Supports metalness/roughness workflow with hardware optimized material data
  - Models - **.gltf** or **.glb** only
    - Requires `POSITION` and `TEXTURE0`
    - Static or skinned
    - Mesh name filtering/renaming/un-naming
  - Scenes
- _Runtime_ Asset `.pak` File:
  - Easy reading of assets
  - Configurable compression
- Rendering:
  - TODO

## Optional features

Screen 13 puts a lot of functionality behind optional features in order to optimize compile time for the most common use cases. The following features are available.

_NOTE_: The deferred and forward renderers have separate code paths and you can choose either on a render-by-render basis.

- **`debug-names`** — Name parameter added to most graphics calls, integrates with your preferred graphics debugger.
- **`deferred-renderer`** *(enabled by default)* — Ability to draw models and lights using a deferred technique.
- **`forward-renderer`** *(enabled by default)* — Same as the deferred renderer, but using a forward technique.

## Content Baking Procedures

### Brotli Compression

Higher compression ratio and somewhat slow during compression. *Currently does not read properly*

```toml
[content]
compression = 'brotli'
buf_size = 4096
quality = 10
window_size = 20
```

### Snap Compression

Faster during compression and lower compression ratio compared to Brotli.

```toml
[content]
compression = 'snap'
```

## History

As a child I was given access to a computer that had GW-Basic; and later one with QBasic. All of my favorite programs started with:

```basic
CLS
SCREEN 13
```

These commands cleared the screen of text and setup a 320x200 256-color paletized color video mode. There were other video modes available, but none of them had the 'magic' of 256 colors.

Additional commands QBasic offered, such as `DRAW`, allowed you to build very simple games incredibly quickly because you didn't have to grok the enirety of linking and compiling in order get things done. I think we should have options like this today, and this project aims to allow future developers to have the same ability to get things done quickly while using modern tools.

## Notes

- Run your game with the `RUST_LOG` environment variable set to `screen_13=trace` for detailed debugging messages
- Make all panics/todos/unreachables and others only have messages in debug builds?
- Consider removing the extra derived things
- Create new BMFont files on Windows using [this](http://www.angelcode.com/products/bmfont/)
- Regenerate files by cd'ing to correct directory and run this:
  - "c:\Program Files (x86)\AngelCode\BMFont\bmfont.com" -c SmallFonts-12px.bmfc -o SmallFonts-12px.fnt
  - "c:\Program Files (x86)\AngelCode\BMFont\bmfont.com" -c SmallFonts-10px.bmfc -o SmallFonts-10px.fnt

https://www.hiagodesena.com/pbr-deferred-renderer.html
https://thomasdeliot.wixsite.com/blog/single-post/2018/04/26/Small-project-OpenGL-engine-and-PBR-deferred-pipeline-with-SSRSSAO

https://www.google.com/search?q=deferred+pbr+pipeline&oq=deferred+pbr+pipeline&aqs=chrome..69i57j0i433i457j0i433j0l3j69i60l2.6933j0j7&sourceid=chrome&ie=UTF-8

http://filmicworlds.com/blog/linear-space-lighting-i-e-gamma/
