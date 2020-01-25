# Miniquad

[![Github Actions](https://github.com/not-fl3/miniquad/workflows/Cross-compile/badge.svg)](https://github.com/not-fl3/miniquad/actions?query=workflow%3A)
[![Crates.io version](https://img.shields.io/crates/v/miniquad.svg)](https://crates.io/crates/miniquad)

Miniquad is safe and cross-platform rendering library focused on portability and low-end platforms support

API is highly inspired by [sokol-gfx](https://github.com/floooh/sokol) ([sokol overview](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html), [2019 update](https://floooh.github.io/2019/01/12/sokol-apply-pipeline.html)). Implementation influenced by [crayon](https://docs.rs/crayon/0.7.1/crayon/video/index.html).

For context management and input on Windows/Linux(and potentially mobiles) "sokol-app" was used. And no external dependencies for WASM. 

For higher level API take a look on:

[good-web-game](https://github.com/not-fl3/good-web-game): implementation of some [ggez](https://github.com/ggez/ggez) subset on top of miniquad, made as compatibility layer to run ggez games on wasm

[macroquad](github.com/not-fl3/macroquad/): raylib-like library on top of miniquad. [100loc arkanoid with macroquad](https://github.com/not-fl3/macroquad/blob/master/examples/arkanoid.rs)

## Supported platforms

* Windows, OpenGl 3
* Linux, OpenGl 3
* WASM, WebGl1 - tested on ios safari, ff, chrome

## Not supported, but desirable platforms

* Android, OpenGl version should be portable enough to run on android, sokol-app code is here and ready, but I just dont have Android phone. 
* Metal. For both MacOs and IOS metal rendering backend next to opengl one is highly desirable. But I just dont have any MacOs capable hardware to start working on it :/ 

## Examples

![Imgur](https://i.imgur.com/TRI50rk.gif)

[examples/quad.rs](https://github.com/not-fl3/miniquad/blob/master/examples/quad.rs): [web](https://not-fl3.github.io/miniquad-samples/quad.html)   
[examples/offscreen.rs](https://github.com/not-fl3/miniquad/blob/master/examples/offscreen.rs): [web](https://not-fl3.github.io/miniquad-samples/offscreen.html)

Worth to mention [zemeroth port](https://not-fl3.github.io/miniquad-samples/zemeroth.html) and [astroblasto](https://not-fl3.github.io/miniquad-samples/astroblasto.html), built with miniquad-powered [good-web-game](https://github.com/not-fl3/good-web-game)

# Building examples

## desktop

```bash
rustup target add x86_64-pc-windows-gnu # for windows cross compilation, this is how windows builds were tested

cargo run --example quad --target x86_64-unknown-linux-gnu
cargo run --example quad --target x86_64-pc-windows-gnu
```

## wasm

```bash
rustup target add wasm32-unknown-unknown
cargo build --example quad --target wasm32-unknown-unknown
```

And then use the following .html to load .wasm:

<details><summary>index.html</summary>

```html
<head>
    <meta charset="utf-8">
    <title>TITLE</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>

<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/miniquad/blob/master/native/sapp-wasm/js/gl.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
    <script>load("quad.wasm");</script> <!-- Your compiled wasm file -->
</body>

</html>
```
</details>

One of the ways to server static .wasm and .html:

```
cargo install basic-http-server
basic-http-server .
```

# Goals

* Fast compilation time. Right now it is ~5s from "cargo clean" for both desktop and web.

* Cross platform. Amount of platform specific user code required should be kept as little as possible.

* Low-end devices support. 

* Hackability. Working on your own game, highly probable some hardware incompability will be found. Working around that kind of bugs should be easy, implementation details should not be hidden under layers of abstraction.

# Non goals

* Ultimate type safety. Library should be entirely safe in Rust's definition of safe - no UB or memory unsafety. But correct GPU state is not type guaranteed. Feel free to provide safety abstraction in the user code than! 

* High end API, like Vulkan/DirectX 12. Take a look on [gfx-rs](https://github.com/gfx-rs/gfx) or [vulkano](https://github.com/vulkano-rs/vulkano) instead!

* sokol-gfx api compatibility. While sokol is absolutely great as an API design foundation, just reimplementing sokol in rust is not a goal. The idea is to learn from sokol, but make a library in a rust way when its possible.

