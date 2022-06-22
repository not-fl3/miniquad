# Miniquad

[![Github Actions](https://github.com/not-fl3/miniquad/workflows/Cross-compile/badge.svg)](https://github.com/not-fl3/miniquad/actions?query=workflow%3A)
[![Docs](https://docs.rs/miniquad/badge.svg?version=0.3.0-alpha)](https://docs.rs/miniquad/0.3.0-alpha/miniquad/index.html)
[![Crates.io version](https://img.shields.io/crates/v/miniquad.svg)](https://crates.io/crates/miniquad)
[![Discord chat](https://img.shields.io/discord/710177966440579103.svg?label=discord%20chat)](https://discord.gg/WfEp6ut)
[![Matrix](https://img.shields.io/matrix/quad-general:matrix.org?label=matrix%20chat)](https://matrix.to/#/#quad-general:matrix.org)

Miniquad is a manifestation of a dream in a world where we do not need a deep dependencies tree and thousands lines of code to draw things with a computer.

Miniquad aims to provide a graphics abstraction that works the same way on any platform with a GPU, being as light weight as possible while covering as many machines as possible. 

## Supported platforms

* Windows, OpenGL 3
* Linux, OpenGL 3
* macOS, OpenGL 3
* iOS, GLES 3
* WASM, WebGl1 - tested on iOS safari, ff, chrome
* Android, GLES3

## Not supported, but desirable platforms

* Android, GLES2 - work in progress.
* Metal. For both MacOs and iOS metal rendering backend next to opengl one is highly desirable. But I just dont have any MacOs capable hardware to start working on it :/

## Examples

![Imgur](https://i.imgur.com/TRI50rk.gif)

[examples/quad.rs](https://github.com/not-fl3/miniquad/blob/master/examples/quad.rs): [web demo](https://not-fl3.github.io/miniquad-samples/quad.html)<br/>
[examples/offscreen.rs](https://github.com/not-fl3/miniquad/blob/master/examples/offscreen.rs): [web demo](https://not-fl3.github.io/miniquad-samples/offscreen.html)<br/>

[PonasKovas/miniquad-mandelbrot](https://github.com/PonasKovas/miniquad-mandelbrot): [web demo](https://ponaskovas.github.io/miniquad-mandelbrot-wasm-demo/)

# Building examples

## linux

```bash
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

## Android

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

* Forkability. Each platform implementation is, usually, just one pure Rust file. And this file is very copy-paste friendly - it doesnt use any miniquad specific abstractions. It is very easy to just copy some part of miniquad's platform implementation and use it standalone.

# Non goals

* Ultimate type safety. Library should be entirely safe in Rust's definition of safe - no UB or memory unsafety. But correct GPU state is not type guaranteed. Feel free to provide safety abstraction in the user code then!

* High end API, like Vulkan/DirectX 12. Take a look on [gfx-rs](https://github.com/gfx-rs/gfx) or [vulkano](https://github.com/vulkano-rs/vulkano) instead!

# Platinum sponsors

Miniquad is supported by:

<p>
  <a href="https://embark-studios.com">
    <img src="https://www.embark.dev/img/logo_black.png" width="201px">
  </a>
</p>
