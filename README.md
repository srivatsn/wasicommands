# Commands 

This repo has some commands that are implemented in Rust and compile to WASI. Currently it implements `echo` and `cat`

To produce the builds run `cargo build` at the root.

To product asyncified builds that can be consumed by VSCode extension, run

```
./binaryen/bin/wasm-opt --asyncify ./target/wasm32-wasi/debug/cat.wasm -o cat.wasm
```