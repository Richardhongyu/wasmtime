// // use std::io::{Read, Write};
// // use std::net::TcpStream;
// // use std::str::from_utf8;
// use anyhow::Result;
// use wasmtime::*;
// use wasmtime_wasi::sync::WasiCtxBuilder;

// fn main() -> Result<()> {
//     let wat = r#"(module
//         (import "wasi_snapshot_preview1" "sock_send" (func $sock_send (param i32 i32 i32 i32 i32) (result i32)))
//         (import "env" "memory" (memory 1 5 shared))
//         (func (export "first_word") (result i32) (i32.load (i32.const 0)))
//     )"#;

//     // Define the WASI functions globally on the `Config`.
//     let engine = Engine::default();
//     let module = Module::new(&engine, wat)?;
//     let mut linker = Linker::new(&engine);
//     wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

//     // Create a WASI context and put it in a Store; all instances in the store
//     // share this context. `WasiCtxBuilder` provides a number of ways to
//     // configure what the target program will have access to.
//     let wasi = WasiCtxBuilder::new()
//         .inherit_stdio()
//         .inherit_args()?
//         .build();
//     let mut store = Store::new(&engine, wat);

//     // Instantiate our module with the imports we've created, and run it.
//     let instance = linker.instantiate(&mut store, &module)?;
//     let foo = instance.get_typed_func::<(), ()>(&mut store, "first_word")?;
//     foo.call(&mut store, ())?;
//     // let foo = instance.get_typed_func::<(), ()>(&mut store, "foo")?;

//     Ok(())

//     // println!("wasm::_start: attempting to make a TCP stream to localhost:3333");
//     // match TcpStream::connect("localhost:8080") {
//     //     Ok(mut stream) => {
//     //         println!("wasm::_start: successfully connected to server in port 3333");

//     //         let msg = b"GET /file.txt HTTP/1.1\r\n\r\n";
//     //         stream.write(msg).unwrap();

//     //         let mut data = [0 as u8; 365];
//     //         match stream.read_exact(&mut data) {
//     //             Ok(_) => {
//     //                 let text = from_utf8(&data).unwrap();
//     //                 println!("wasm::_start: received {}", text);
//     //             }
//     //             Err(e) => {
//     //                 println!("wasm::_start: failed to receive data: {}", e);
//     //             }
//     //         }
//     //     }
//     //     Err(e) => {
//     //         println!("wasm::_start: failed to connect: {}", e);
//     //     }
//     // }
//     // println!("wasm::_start: exit");
// }

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
fn main() -> wasmtime::Result<()> {
    let wat = r#"(module
        (import "wasi_snapshot_preview1" "sock_send" (func $sock_send (param i32 i32 i32 i32 i32) (result i32)))
        ;; (import "env" "memory" (memory 1 5 shared))
        (memory (export "memory") 2 3)
        (func (export "first_word") (i32.load (i32.const 0)) drop)
    )"#;
    // Compile our module and create a `Linker` which has WASI functions defined
    // within it.
    let engine = Engine::default();
    let module = Module::new(&engine, wat)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;
    // Configure and create a `WasiCtx`, which WASI functions need access to
    // through the host state of the store (which in this case is the host state
    // of the store)
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    // Instantiate our module with the imports we've created, and run it.
    let instance = linker.instantiate(&mut store, &module)?;
    let foo = instance.get_typed_func::<(), ()>(&mut store, "first_word")?;
    foo.call(&mut store, ())?;
    Ok(())
}
