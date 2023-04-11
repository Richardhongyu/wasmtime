use wasi;

fn main() -> Result<()> {
    
    wasi::sock_recv()
    // let wat = r#"(module
    //     (import "wasi_snapshot_preview1" "sock_send" (func $sock_send (param i32 i32 i32 i32 i32) (result i32)))
    //     ;; (import "env" "memory" (memory 1 5 shared))
    //     (memory (export "memory") 2 3)
    //     (func (export "first_word") (i32.load (i32.const 0)) drop)
    // )"#;
    // // Compile our module and create a `Linker` which has WASI functions defined
    // // within it.
    // let engine = Engine::default();
    // let module = Module::new(&engine, wat)?;
    // let mut linker = Linker::new(&engine);
    // wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;
    // // Configure and create a `WasiCtx`, which WASI functions need access to
    // // through the host state of the store (which in this case is the host state
    // // of the store)
    // let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    // let mut store = Store::new(&engine, wasi_ctx);
    // // Instantiate our module with the imports we've created, and run it.
    // let instance = linker.instantiate(&mut store, &module)?;
    // let foo = instance.get_typed_func::<(), ()>(&mut store, "first_word")?;
    // foo.call(&mut store, ())?;
    // Ok(())
}
