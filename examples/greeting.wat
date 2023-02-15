(module
    (import "wasi_snapshot_preview1" "rdma_list" (func $rdma_list (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    (func $main (export "_start")
        (call $rdma_list)
        return
    )
)