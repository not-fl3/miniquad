# Intro

A short intro into the current state of wasm:

`cargo build --target wasm32-unknown-unknown`
will produce `.wasm` file. Wasm is a way to script a web pages in a very special way.

One .wasm file is one "module". The module is very similar to `.o` or `.dll` files on native platforms - its a dynamically loadable library with some functions.

(what actually is inside wasm file)[https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format]

Each wasm module has two lists: import functions and export functions. 
Export functions: functions exported from wasm that can be called later from JS. 
Import functions: JS functions that can be called from any WASM function.

"wasm" can't be included in the web page with <script> tag. "wasm" can be loaded with some special JS code. 

That loader code is doing this:
- load wasm binary data - download the file from the internet and get bytes
- fill import functions table. all the JS functions that wasm use should be in that one table
- call browser api to instantiate wasm with the given import table
- get references into wasm's export function and wasm memory. Now JS can call our wasm! 

In miniquad's case that JS loader code will also call "main" function from wasm export and will forward JS events as appropriate WASM functions calls.


# Basic usage

To get wasm module with all necessary export functions: just add "miniquad" as a dependency in Cargo.toml.
Then to load this wasm module use miniquad's JS wasm loader. Right now its named as "gl.js" for historical reasons:
```
<script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script> <!-- gl.js from miniquad repo (native/sapp-wasm/js/gl.js) -->
<script>load("quad.wasm");</script> <!-- Your compiled wasm file -->
```

This is enough for all pure rust miniquad-based applications. In other words: if your dependency use miniquad for web and claims web support - just use miniquad's loader to run your app on wasm. For example - if you use [good-web-game](https://github.com/not-fl3/good-web-game) - you will need miniquad's loader.


# Advances linking

Sometimes miniquad's API is not enough and custom javascript is needed.

miniquad's gl.js will provide two global variables: "wasm_exports" and "wasm_memory".

All rust's "#[no_mangle] pub extern "C" fn function_name() {}" functions will be available to call from "wasm_exports".
```
wasm_exports.my_rust_function(1, 2, 3); // make a wasm call from JS
```
To call js from rust, however, some initialization work need to be done. All JS functions available to wasm should be explicitly listed before wasm loading. Before "load" call in our case. 
The set of JS functions available to call from rust is called Plugin in miniquad's terminology.

Each plugin has two functions:

- *register*. Will be called before wasm initialization. Can add additional function to wasm's import table: to make plugins JS code available for wasm.
- *set_wasm_refs*. Will be called after successful wasm initialization and to allow plugin store wasm's export table and wasm's memory - to call any rust function available later in JS.

To add plugin call "miniquad_add_plugin" from plugin's JS file. "gl.js" should be already imported with <script> in the current web page. 
  
Minimal example: 

main.rs:

```rust
extern "C" {
  fn hi_from_js();
}

#[no_mangle]
extern "C" fn hi_from_rust() {
  // we can call JS from rust!
  hi_from_js(); 
}

struct Stage;
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}
    fn draw(&mut self, _ctx: &mut Context) {}
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| {      
      UserData::owning(Stage, ctx))
    }};
}
```

```plugin.js
register_plugin = function (importObject) {
    importObject.env.hi_from_wasm = function (js_object) {
        console.log("hi")
    }
}

document.onclick = function () {
    // and rust from JS!
    wasm_exports.hi_from_rust();
};

// miniquad_add_plugin receive an object with two fields: register_plugin and on_init. Both are functions, both are optional.
miniquad_add_plugin({register_plugin});

```

index.html
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
    <script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
    <script src="plugin.js"></script>
    <script>load("sapp-jsutils.wasm");</script>
</body>

</html>
```

The very good thing about WASM - that everything is super transparent and straightforward. console.log(wasm_exports) will give a very clear picture of whats going on, what functions are available. And so on - each object is debug friendly.

# Type system helpers

Now we know how to call JS from rust and Rust from JS. 
The problem - functions are very limited in available types. Only f32/f64, i8/u8, i32/u32 (and not i64/u64) and pointers are available.
Surprisingly usually it is enough - for games usually there are not many FFI functions and rolling your own buffer converter from wasm memory to JS memory is good enough.
But there is `sapp-jsutils` plugin available that can help with working with strings or even arbitrary JS objects. 

With `sapp-utils` rust code may look like this: 
```
#[no_mangle]
pub extern "C" fn hi_rust(js_object: JsObject) {
    let mut message = String::new();

    js_object.to_string(&mut message);
    miniquad::debug!("{}", message);
}
```

For more info check [demo project](https://github.com/not-fl3/miniquad-js-interop-demo.git). This example showcase usage of strings, arrays and structs bi-directional usage - complex types are used in both argument and return positions for both JS and Rust calls.