use anyhow::Result;

wiggle::from_witx!({
    witx: ["$WASI_ROOT/spec/witx/wasi_rdma.witx"],
});
