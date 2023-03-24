# wasi-rdma

## build the wasm32-wasi target

```bash
cargo build --release --target=wasm32-wasi
```

## run the example with the wasi-rdma interface

```bash
../../../../target/debug/wasmtime --wasi-modules experimental-wasi-rdma target/wasm32-wasi/release/rdma_test.wasm 
```

