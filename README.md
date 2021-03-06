[![Build Status](https://img.shields.io/github/workflow/status/MFEK/glif/Rust?label=Rust&logo=Rust)](https://github.com/MFEK/glif/actions?workflow=Rust)

# glif

Glyph editor for the Modular Font Editor K project.

<img src="https://raw.githubusercontent.com/MFEK/glif/master/doc/screenshot.png" width="300"><img src="https://raw.githubusercontent.com/MFEK/glif/master/doc/screenshot2.png" width="300">

## Overview

MFEKglif mixes three technologies: Skia, a powerful path rasterizer and manipulation library; Dear ImGui, an immediate mode GUI toolkit; and Rust, a modern high-performance systems language.

I wrote it after, hopefully, learning from the mistakes made by George Williams in FontForge, after being a user of FontForge for six years and a co-maintainer for one and a half years.

MFEKglif is the flagship program of the Modular Font Editor K project, which aims to create a full font editor by making many small programs that all work together, fulfilling the Unix adage that each program should have one task and do that task well. MFEKglif aims to do the task of creating and editing glyphs well.

To make this as easy as possible to build, and cross-platform without hassle, resources are compiled into the binary via the Rust `include_str!` macro, and MFEKglif is statically compiled.

## Keys

Note: This is a basic list to get you started. A complete list can be found in `resources/default_keymap.xml`. You may copy this file to e.g. `$HOME/.config/MFEK/glif/keybindings.xml` on Linux and modify it.

### I/O
* <kbd>Ctrl</kbd><kbd>O</kbd> &mdash; Open user-specified .glif file
* <kbd>Ctrl</kbd><kbd>S</kbd> &mdash; Save current glyph in a multi-layered .glif
* <kbd>Ctrl</kbd><kbd>U</kbd> &mdash; Flatten the topmost layer, and write it to a user-specified .glif file
* <kbd>Ctrl</kbd><kbd>E</kbd> &mdash; Export the multi-layered .glif to different `glyphs/` directories for each layer, with `layerinfo.plist` and update `layercontents.plist` for each.

### Tools
* <kbd>A</kbd> &mdash; Select &laquo;Pan&raquo; tool
* <kbd>P</kbd> &mdash; Select &laquo;Pen&raquo; tool
* <kbd>V</kbd> &mdash; Select &laquo;Select&raquo; tool
* <kbd>Z</kbd> &mdash; Select &laquo;Zoom&raquo; tool
* <kbd>W</kbd> &mdash; Select &laquo;Variable Width Stroke&raquo; tool
* <kbd>M</kbd> &mdash; Select &laquo;Measure&raquo; tool
* <kbd>N</kbd> &mdash; Select &laquo;Anchors&raquo; tool
* <kbd>S</kbd> &mdash; Select &laquo;Shapes&raquo; tool

### Selection
* <kbd>Ctrl</kbd><kbd>A</kbd> &mdash; Select all points in current layer
* <kbd>Backspace</kbd> &mdash; Delete currently selected points

## Running from artifacts

MFEKglif is still alpha-quality software, and a numbered release hasn't been made yet. Before 1.0 is out, though, you can test it out with the artifacts function in GitHub. Go to [«Actions»](https://github.com/MFEK/glif/actions), choose a commit, and download the artifact for your OS. Three are uploaded: MFEKglif-linux, MFEKglif-windows, and MFEKglif-macos (not notarized).

### Note for Windows users

MFEKglif currently does not have the ability to write errors or warnings to the screen through dialog boxes or its GUI, which most Windows applications do and are expected to do. We write them to `stderr`, as Unix applications do. We will eventually use MFEKglif's built in console (which appears when you press <kbd>:</kbd>), but for now for best results, and to see the error if you get a crash or MFEKglif does not open, please run MFEKglif through `MSYS2` or `Cmder`.

## Building

### Mac users

Apple charges a fee to "notarize" applications and without this "notarization" MFEKglif will not run correctly, or in some cases, at all. So, for the foreseeable future, you must _build MFEKglif from source on OS X_. This is not as hard as it sounds! :-)

* Download and install the [Vulkan SDK](https://vulkan.lunarg.com/).

### Linux users

MFEKglif depends on GTK3 (for the open/save dialogs). If using X11 and not Wayland, it depends on the X11 C shape extension (`libxcb-shape.so.0`) and the xfixes extension (`libxcb-xfixes.so.0`). Their header files are also needed: `/usr/include/xcb/shape.h` and `/usr/include/xcb/xfixes.h`.

On Arch Linux, these two packages provide all the dependencies: `gtk3` `libxcb`

On Ubuntu, these three packages provide the dependencies: `libgtk-3-dev` `libxcb-shape0-dev` `libxcb-xfixes0-dev`

### For everyone

* Download and install [`rustup`](https://rustup.rs/), selecting the `nightly` toolchain.
* Pull this repository, and finally
* Run the below command to get started.

### Errors?

If you previously pulled the repository and get errors related to `glifparser`, `mfek-ipc`, or another local unstable dependency, try running `cargo update` to force Cargo to pull the latest versions from GitHub.

## Contributing

I typically build and run MFEKglif like this:

```
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- examples/Q_.glif
```

We welcome all contributions! Please open an issue first so we can discuss before you make big changes so no effort is wasted.

### More debug output

It is possible to get even more debug output out of MFEKglif for figuring out where problems lie. To ask MFEKglif to dump the parsed .glif file on runtime, pass `DEBUG_DUMP_GLYPH=Y`. To see every single `sdl2` event (warning: this will flood your stdout) pass `DEBUG_EVENTS=Y`.

### Goals

Contributions which do not work on at least GNU/Linux and Windows will be rejected; we want to be able to build MFEKglif on as many platforms as possible. Both Skia and Dear ImGui are cross-platform; we use Vulkan and not OpenGL so we are future-proof even on OS X.

Contibutions will also be judged on how well they fit into the MFEK project as a whole. It's possible that your idea fits better into another module and not MFEKglif; we can help you figure out where it should go.

## License

```
Copyright 2020–2021 Fredrick R. Brennan, Matthew Blanchard & MFEK Authors

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this software or any of the provided source code files except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied.  See the License for the
specific language governing permissions and limitations under the License.
```

**By contributing you release your contribution under the terms of the license.**
