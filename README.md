# Miniquad

Cross platform context managment and OpenGL wrapper.   

API is highly inspired by [sokol-gfx](https://github.com/floooh/sokol) ([sokol overview](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html), [2019 update](https://floooh.github.io/2019/01/12/sokol-apply-pipeline.html)). Implementation influenced by [crayon](https://docs.rs/crayon/0.7.1/crayon/video/index.html).

For context management and input on Windows/Linux(and potentially mobiles) "sokol-app" was used. And no external dependencies for WASM. 

## Supported platforms

* Windows, OpenGl 3
* Linux, OpenGl 3
* WASM, WebGl1 - tested on ios safari, ff, chrome

## Not supported, but desirable platforms

* Android, OpenGl version should be portable enough to run on android, sokol-app code is here and ready, but I just dont have Android phone. 
* Metal. For both MacOs and IOS metal rendering backend next to opengl one is highly desirable. But I just dont have any MacOs capable hardware to start working on it :/ 

## Examples

![Imgur](https://i.imgur.com/xp5xc7j.gif)

[examples/quad.rs](https://github.com/not-fl3/miniquad/blob/master/examples/quad.rs): [web](https://not-fl3.github.io/miniquad-samples/quad.html)   
[examples/offscreen.rs](https://github.com/not-fl3/miniquad/blob/master/examples/offscreen.rs): [web](https://not-fl3.github.io/miniquad-samples/offscreen.html)

Worth to mention [zemeroth port](https://not-fl3.github.io/miniquad-samples/zemeroth.html) and [astroblasto](https://not-fl3.github.io/miniquad-samples/astroblasto.html), built with miniquad-powered 

# Building examples

## desktop

```bash
rustup target add x86_64-pc-windows-gnu # for windows cross compilation, this is how windows builds was tested

cargo run --example quad --target x86_64-unknown-linux-gnu
cargo run --example quad --target x86_64-pc-windows-gnu
```

## wasm

```bash
rustup target add wasm32-unknown-unknown
cargo build --example quad --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/examples/quad.wasm js
cd js/ #  and launch http server with wasm MIME, maybe check index.html to match wasm name to load
```

# Goals

* Fast compilation time. Right now it is ~5s from "cargo clean" for both desktop and web.

* Cross platform. Amount of platform specific user code required should be kept as little as possible.

* Low-end devices support. 

* Hackability. Working on your own game, highly probable some hardware incompability will be found. Working around that kind of bugs should be easy, implementation details should not be hidden under layers of abstraction.

# Non goals

* Ultimate safety. Safe functions should be safe in Rust's definition of safe, but some things may be unsafe and will be marked with "unsafe". Fill free to provide safety abstraction in the user code than! 

* High end API, like Vulkan/DirectX 12. Take a look on [gfx-rs](https://github.com/gfx-rs/gfx) or [vulkano](https://github.com/vulkano-rs/vulkano) instead!

* sokol-gfx api compatibility. While sokol is absolutely great as an API design foundation, just reimplementing sokol in rust is not a goal. The idea is to learn from sokol, but make a library in a rust way when its possible.

