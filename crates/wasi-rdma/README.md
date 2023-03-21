# wasi-rdma

## enable the wasi-rdma

the `wasi-rdma` feature is enabled defaultly for now, so you can just compile wasmtime and get a wasmtime binary equipped with `wasi-rmda` interface

```bash
cargo build
```

## run the example 

edit the `greeting.wat` file

```wat
(module
    ;; (import "wasi_snapshot_preview1" "rdma_list" (func $rdma_list (result i32)))
    (import "wasi_ephemeral_rdma" "print_hello_world" (func $hello_world (result)))

    (memory 1)
    (export "memory" (memory 0))

    (func $main (export "_start")
        (call $hello_world)
        return
    )
)
```

then generate the `greeting.wasm` file

```bash
wat2wasm examples/greeting.wat
```

use the `wasi-rdma` interface to run the wasm file

```bash
./target/debug/wasmtime --wasi-modules experimental-wasi-rdma greeting.wasm
```

the output is
```bash
[Debug] you are using wasi-rdma interface
Hello World!
```