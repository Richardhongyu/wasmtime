(module
    (import "wasi_snapshot_preview1" "sock_send" (func $sock_send (param i32 i32 i32 i32 i32) (result i32)))
    ;; (import "wasi_ephemeral_rdma" "print_hello_world" (func $hello_world (result)))

    (memory 1)
    (export "memory" (memory 0))

    (func $main (export "_start")
        (call $sock_send
	      (i32.const 1)
	      (i32.const 1)
	      (i32.const 1)
	      (i32.const 1)
	      (i32.const 0))
        return
    )
)
