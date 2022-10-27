# Compiling from Rust to WebAssembly

There are two main use cases for Rust and WebAssembly:

1. Building an entire application -- an entire web application based in Rust.
2. Building a part of an application -- using Rust in an existing JavaScript frontend.

The Rust team is currently focusing on the latter case and for most cases it will be sufficient. However, existing efforts like [yew](https://github.com/yewstack/yew) exist for building an entire application with Rust.

For this tutorial, we will build packages using `wasm-pack`, a tool for building JavaScript packages in Rust. Install the binary with `cargo install`.

```bash
cargo install wasm-pack
```

## Building a WebAssembly package

```bash
cargo new --lib hello-wasm
```
