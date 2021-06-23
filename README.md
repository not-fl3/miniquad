# Miniquad

[![Github Actions](https://github.com/not-fl3/miniquad/workflows/Cross-compile/badge.svg)](https://github.com/not-fl3/miniquad/actions?query=workflow%3A)
[![Docs](https://docs.rs/miniquad/badge.svg?version=0.3.0-alpha)](https://docs.rs/miniquad/0.3.0-alpha/miniquad/index.html)
[![Crates.io version](https://img.shields.io/crates/v/miniquad.svg)](https://crates.io/crates/miniquad)
[![Discord chat](https://img.shields.io/discord/710177966440579103.svg?label=discord%20chat)](https://discord.gg/WfEp6ut)
[![Matrix](https://img.shields.io/matrix/quad-general:matrix.org?label=matrix%20chat)](https://matrix.to/#/#quad-general:matrix.org)

Miniquad is safe and cross-platform rendering library focused on portability and low-end platforms support

API is highly inspired by [sokol-gfx](https://github.com/floooh/sokol) ([sokol overview](https://floooh.github.io/2017/07/29/sokol-gfx-tour.html), [2019 update](https://floooh.github.io/2019/01/12/sokol-apply-pipeline.html)). Implementation influenced by [crayon](https://docs.rs/crayon/0.7.1/crayon/video/index.html).

For context management and input on Windows/Linux(and potentially mobiles) "sokol-app" was used. And no external dependencies for WASM.

For higher level API take a look on:

[good-web-game](https://github.com/not-fl3/good-web-game): implementation of some [ggez](https://github.com/ggez/ggez) subset on top of miniquad, made as compatibility layer to run ggez games on wasm

[macroquad](https://github.com/not-fl3/macroquad): raylib-like library on top of miniquad. [100loc arkanoid with macroquad](https://github.com/not-fl3/macroquad/blob/master/examples/arkanoid.rs)

## Supported platforms

* Windows, OpenGl 3
* Linux, OpenGl 3
* macOS, OpenGL 3
* iOS, GLES 3
* WASM, WebGl1 - tested on ios safari, ff, chrome
* Android, GLES3

## Not supported, but desirable platforms

* Android, GLES2 - work in progress.
* Metal. For both MacOs and IOS metal rendering backend next to opengl one is highly desirable. But I just dont have any MacOs capable hardware to start working on it :/

## Examples

![Imgur](https://i.imgur.com/TRI50rk.gif)

[examples/quad.rs](https://github.com/not-fl3/miniquad/blob/master/examples/quad.rs): [web demo](https://not-fl3.github.io/miniquad-samples/quad.html)<br/>
[examples/offscreen.rs](https://github.com/not-fl3/miniquad/blob/master/examples/offscreen.rs): [web demo](https://not-fl3.github.io/miniquad-samples/offscreen.html)<br/>

[PonasKovas/miniquad-mandelbrot](https://github.com/PonasKovas/miniquad-mandelbrot): [web demo](https://ponaskovas.github.io/miniquad-mandelbrot-wasm-demo/)

Worth to mention [zemeroth port](https://not-fl3.github.io/miniquad-samples/zemeroth.html) and [astroblasto](https://not-fl3.github.io/miniquad-samples/astroblasto.html), built with miniquad-powered [good-web-game](https://github.com/not-fl3/good-web-game)

# Building examples

## linux

```bash
# ubuntu system dependencies
apt install libx11-dev libxi-dev libgl1-mesa-dev

cargo run --example quad
```

## windows

```bash
# both MSVC and GNU target is supported:
rustup target add x86_64-pc-windows-msvc
# or
rustup target add x86_64-pc-windows-gnu

cargo run --example quad
```

## wasm

```bash
rustup target add wasm32-unknown-unknown
cargo build --example quad --target wasm32-unknown-unknown
```

And then use the following .html to load .wasm:

<details><summary>index.html</summary>

```html
<html lang="en">

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

## android

Recommended way to build for android is using Docker.<br/>
miniquad use slightly modifed version of `cargo-apk`

```
docker run --rm -v $(pwd)":/root/src" -w /root/src notfl3/cargo-apk cargo quad-apk build --example quad
```

APK file will be in `target/android-artifacts/(debug|release)/apk`

With "log-impl" enabled all log calls will be forwarded to adb console.
No code modifications for Android required, everything should just works.

## iOS

See miniquad iOS [sample project](https://github.com/Gordon-F/miniquad_ios_example).

## cross compilation

```bash

# windows target from linux host:
# this is how windows builds are tested from linux machine:
rustup target add x86_64-pc-windows-gnu
cargo run --example quad --target x86_64-pc-windows-gnu
```

# Goals

* Fast compilation time. Right now it is ~5s from "cargo clean" for both desktop and web.

* Cross platform. Amount of platform specific user code required should be kept as little as possible.

* Low-end devices support.

* Hackability. Working on your own game, highly probable some hardware incompability will be found. Working around that kind of bugs should be easy, implementation details should not be hidden under layers of abstraction.

# Non goals

* Ultimate type safety. Library should be entirely safe in Rust's definition of safe - no UB or memory unsafety. But correct GPU state is not type guaranteed. Feel free to provide safety abstraction in the user code then!

* High end API, like Vulkan/DirectX 12. Take a look on [gfx-rs](https://github.com/gfx-rs/gfx) or [vulkano](https://github.com/vulkano-rs/vulkano) instead!

* sokol-gfx api compatibility. While sokol is absolutely great as an API design foundation, just reimplementing sokol in rust is not a goal. The idea is to learn from sokol, but make a library in a rust way when it is possible.

# Platinum sponsors

Miniquad is supported by:

<p>
  <a href="https://embark-studios.com">
    <img src="https://www.embark.dev/img/logo_black.png" width="201px">
  </a>
</p>
