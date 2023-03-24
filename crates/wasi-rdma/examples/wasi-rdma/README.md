# wasi-rdma

```bash
target/release/witx-bindgen source_flie.witx > target_file.rs

# example
target/release/witx-bindgen ../wasmtime_workdir/wasmtime/crates/wasi-rdma/spec/witx/wasi_ephemeral_rdma.witx > ../wasmtime_workdir/wasmtime/crates/wasi-rdma/examples/wasi-rdma/src/generated.rs 
```