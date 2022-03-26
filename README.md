# Yew Learning Diary

This repo documents my journey in learning how to develop web applications with the Yew framework for Rust WebAssembly.

## Resources

[Rust Language]("https://rust-lang.org)

[Yew Framework]("https://yew.rs)

[Trunk]("https://trunkrs.dev)

## Project Setup

### Overview

There are 3 things you need to do to write, compile, build, debug, and package an application using Yew.

- install Rust
- install WebAssembly target
- install Trunk

### Install Rust

Rust is pretty easy to install. Follow the [Official Instructions](https://www.rust-lang.org/tools/install) to get that done.

### Install WebAssembly Target

Rust can compile source code for several different targets (i.e. processor architectures). The compliation target for browser-based WebAssembly is known as "wasm32-unknown-unknown". Issue the following command to add this target:

```
rustup target add wasm32-unknown-unknown
```

### Install Trunk

Trunk is a tool for managing, packaging, and deployment of apps developed with Yew.

```
cargo install trunk
```
