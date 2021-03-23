# Cross-platform logs with miniquad

Miniquad is cross-platform layer between rust code and target platforms rendering API. 
Console API necessary for logs are some kind of output API and its on miniquads responsibility to abstract it away. On the other hand, using `log-rs` with different frontends are the most common way to do logging in rust cosystem. 

So there are two different ways to send log messages into console with miniquad:

## 1. With log-rs

Use `debug!`, `warn!`, `trace!`, `info!` macroses from `log-rs` just like with any other library. The only issue - special logging frontend for wasm will be needed. Fortunately, there is one: `sapp-console-log`.

dependencies to Cargo.toml:
```toml
[dependencies]
log = "0.4"
[target.wasm32-unknown-unknown.dependencies]
sapp-console-log = "0.1"
[target.'cfg(not(target_arch = "wasm32")'.dependencies]
env_logger = "0.7" # any other log-rs frontend will work fine as well
```

initialization in main.rs:
```rust
fn main() {
    #[cfg(target_arch = "wasm32")]
    sapp_console_log::init().unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    
    info!("Logging is initialized!"); // this will be `console.info` on web or handled and filtred by env_logger into stderr
}
```

# 2. With embedded logging implementation

If the main target platform is wasm - there are not much of a choice, just `sapp_console_log`. 
miniquad can provide its own logging macroses, very similar to `log-rs` ones.
Each logging call will be redirected into appropriate `console.*()` call on wasm and just into `eprintln!()` on desktop.

To use this, enable miniquad "log-impl" feature in Cargo.toml:
```toml
#[dependencies]
miniquad = { version = "0.2", features = [ "log-impl" ]}
```

And then all logging macroses will be available just from miniquad:
```rust
use miniquad::info;

fn main() {
    info!("Logging is available!"); // this will be `console.info` on web and `eprintln!` on desktop
}
```