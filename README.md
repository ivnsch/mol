# mol

![demo](./img/demo.gif)

A minimal molecule explorer.

File formats:

- mol2

Contains too some experiments, like the linear alkane generator shown above.

## Instructions

#### Web app

This repository contains a Rust library and a next.js app that uses this library. Building the next.js app will build the Rust library and copy the wasm files to the right folder.

```
npm run build
npm run dev
```

Note: `--release` in ./package.json currently mandatory
https://github.com/bevyengine/bevy/issues/16030

The library is independent from the web app and can be used too by a Rust binary:

#### Rust standalone library

```
mol = { git = "https://github.com/ivnsch/mol.git" }

```

```rust
mol::init_sim();
```
