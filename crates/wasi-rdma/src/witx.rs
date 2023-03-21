wiggle::from_witx!({
    witx: ["$WASI_ROOT/witx/wasi_ephemeral_rdma.witx"],
    // errors: { rdma_error => WasiRdmaError }
});

impl wiggle::GuestErrorType for types::RdmaError {
    fn success() -> Self {
        Self::Success
    }
}
