# wasi-rdma

## enable the wasi-rdma

the `wasi-rdma` feature is enabled defaultly for now, so you can just compile wasmtime and get a wasmtime binary equipped with `wasi-rmda` interface

```bash
cargo build
```

## generate the bindings

generate the user called bindings for the `wasi_ephemeral_rdma` interface
```
see [../examples/wasi-rdma/README.md] for more details
```
<!-- ```bash
witx-bindgen --language rust --output binding/lib_generated.rs src/wasi_ephemeral_rdma.witx
``` -->

expand the wiggle bindings for the `wasi_ephemeral_rdma` interface
```bash
cargo expand witx > ./src/binding/_macro_expanded_lib.rs
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

## todolist

1. fix the `TODO:` in the code.  
    - [ ] error handler  
    - [ ] send flags  
2. change the hardcode number
    - [ ] compare the `max_inline_data` with 32/16
    - [ ] change the length of `rdma_post_send` and `rdma_post_recv`
3. `test_server_client.sh`
    - [ ] adjust the buffer length `send_msg` and `recv_msg` in the bash
    - [ ] find the ip address of nic and pass it to the client and server
4. test the overhead(the specific experiments can be seen in this [doc](https://docs.google.com/presentation/d/1Qg7Ns0HlSDUIdAkiS5_w7WXc56n-kcxijS1lM3qazKI/edit?usp=sharing))
    - [ ] use wasi-socket to write/read a large file to another wasm