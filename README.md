# mol

![demo](./img/demo.gif)

A minimal visualizer of linear alkanes, allows to set any number of carbons, and some interactions.

### Instructions
```
cargo run
```

Web:

```
wasm-pack build --out-dir ./wasm --debug
```

Then move the created files in `./wasm` to the wasm folder (in the root directory) of the next app. Restart next app if needed.

Note: this is a temporary setup, the next app will probably be added directly to this repository.
