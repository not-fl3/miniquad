# How WASM works

## Intro

A short intro into the current state of wasm:

`cargo build --target wasm32-unknown-unknown` will produce `.wasm` file. Wasm is a way to script a web pages in a very special way.

One `.wasm` file is one "module". The module is very similar to `.o` or `.dll` files on native platforms - its a dynamically loadable library with some functions.

(what actually is inside wasm file)[https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format]

Each wasm module has two lists: import functions and export functions. 
* **Export functions:** functions exported from wasm that can be called later from JS. 
* **Import functions:** JS functions that can be called from any WASM function.

"wasm" can't be included in the web page with `<script>` tag. "wasm" can be loaded with some special JS code. 

That loader code is doing this:
- Load wasm binary data - download the file from the internet and get bytes.
- Fill import functions table. all the JS functions that wasm use should be in that one table.
- Call browser api to instantiate wasm with the given import table.
- Get references into wasm's export function and wasm memory. Now JS can call our wasm!

In miniquad's case that JS loader code will also call `main` function from wasm export and will forward JS events as appropriate WASM functions calls.


## Basic usage

To get wasm module with all necessary export functions: just add `miniquad` as a dependency in `Cargo.toml`. Then to load this wasm module use miniquad's JS wasm loader. Right now its named as `gl.js` for historical reasons:
```
<script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script> <!-- gl.js from miniquad repo (native/sapp-wasm/js/gl.js) -->
<script>load("quad.wasm");</script> <!-- Your compiled wasm file -->
```

This is enough for all pure rust miniquad-based applications. In other words: if your dependency use miniquad for web and claims web support - just use miniquad's loader to run your app on wasm. For example - if you use [macroquad](https://github.com/not-fl3/macroquad) - you will need miniquad's loader.
