wiggle::from_witx!({
    witx: ["$WASI_ROOT/witx/wasi_ephemeral_rdma.witx"],
});

impl wiggle::GuestErrorType for types::RdmaError {
    fn success() -> Self {
        Self::Success
    }
}
