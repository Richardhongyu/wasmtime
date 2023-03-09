use anyhow::Result;
pub struct WasiRdmaCtx{}
wiggle::from_witx!({
    witx: ["$WASI_ROOT/spec/witx/wasi_rdma.witx"],
});
impl Rdma for WasiRdmaCtx {
}
