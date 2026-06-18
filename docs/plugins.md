# Plugins

There is exists plugin system for miniquad, and for any library that uses miniquad as a base, like [macroquad](https://github.com/not-fl3/macroquad), [good-web-game](https://github.com/not-fl3/good-web-game). Not all features should be in miniquad itself, therefore some functionality like audio, networking and etc are moved out as a plugins.

Plugin is combination of crate and `.js` file. On native platforms plugin may just work. But for wasm, there is extra work to be done.

# How to use plugin

Add plugin to your dependencies in `Cargo.toml` file:

```
plugin = "0.1.4"
```

For wasm, you should add plugin loading after loading of `gl.js` file, and before wasm loading: 

```diff
 <script src="gl.js"></script>
+<script src="my_plugin.js"></script>
 <script>load("demo.wasm");</script>
```

Plugin will do all the other work by itself.

If versions of `.js` and rust code are different, error will be written to the browser console.

# List of plugins

List of all plugins can be found [here](https://github.com/ozkriff/awesome-quads#plugins).

# How to write plugin

You could take a look at [`sapp-jsutils`](https://github.com/not-fl3/sapp-jsutils/) and [`miniquad-js-interop-demo`](https://github.com/not-fl3/miniquad-js-interop-demo).

## JS

In `JS` you must call `miniquad_add_plugin` with object with these fields:
```js
miniquad_add_plugin({
    register_plugin: my_register_js_plugin, // Function where you add bindings. Will be called before wasm initialization. Can add additional function to wasm's import table: to make plugins JS code available for wasm.
    on_init: my_on_init, // Your function where you initialize all your data. Will be called after successful wasm initialization and to allow plugin store wasm's export table and wasm's memory - to call any rust function available later in JS.
    name: "quad_url", // This name must contain `_` instead of `-`, because next it will be used to call function to get current version.
    version: "0.1.0" // This version must be updated with crate update
});
```

* `on_init` function receives `(wasm_memory, wasm_exports)` and should return nothing.
* `register_plugin` receives `(importObject)` and must return nothing, but modify this `importObject`.

### Rust → JS

In `register_plugin` function you just write function that Rust can call: 
```js
my_register_js_plugin = function (importObject) {
    importObject.env.crate_name_random = function (seed) {
        return 42 + seed;
    }
}
```

On Rust side this function will look like:
```rust
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn crate_name_random(seed: u32) -> u32;
}
```

Name of function should start from you crate name, to avoid collisions.

### JS → Rust

To call Rust function from JS you should use `wasm_exports` which you receives where `on_init` function is called:
```JS
wasm_exports.crate_name_debug_number(42);
```

On Rust side it must look like: 

```rust
#[no_mangle]
#[cfg(target_arch = "wasm32")]
extern "C" fn crate_name_debug_number(number: u32) {
	dbg!(number);
}
```

The very good thing about WASM - that everything is super transparent and straightforward. `console.log(wasm_exports)` will give a very clear picture of whats going on, what functions are available. And so on - each object is debug friendly.

## Type system helpers

Now we know how to call JS from rust and Rust from JS. The problem - functions are very limited in available types. Only `f32`/`f64`, `i8`/`u8`, `i32`/`u32` (and not `i64`/`u64`) and pointers are available. Surprisingly usually it is enough - for games usually there are not many FFI functions and rolling your own buffer converter from wasm memory to JS memory is good enough. But there is [`sapp-jsutils`](https://github.com/not-fl3/sapp-jsutils/) plugin available that can help with working with strings or even arbitrary JS objects. 

With `sapp-utils` rust code may look like this: 
```rust
#[no_mangle]
pub extern "C" fn hi_rust(js_object: JsObject) {
    let mut message = String::new();

    js_object.to_string(&mut message);
    miniquad::debug!("{}", message);
}
```

For more info check [demo project](https://github.com/not-fl3/miniquad-js-interop-demo). This example showcase usage of strings, arrays and structs bi-directional usage - complex types are used in both argument and return positions for both JS and Rust calls.

### Version

You must add function to determine your crate Rust code version:
```rust
#[no_mangle]
extern "C" fn <crate name>_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}
```

`<crate name>` must be equal to name you specified in `miniquad_add_plugin`.

## Readme

If you decided to move your code to separate crate, then in readme of your plugin you should write:
* Link to this documentation on how to use plugins.
* Which plugins are used with their versions, preferably with links to right version of `.js` file.
* Link to current version of your `.js` file.

Or, if your library is not a plugin, but it use plugins, you should write this too.
