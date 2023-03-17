mod table;
mod lib;
mod rdma;
use wiggle::{from_witx, GuestError};

use crate::wasi_rdma_wasmtime::types::RdmaError;
from_witx!({
    witx: ["/home/lx/wasmtime/crates/wasi-rdma/spec/witx/wasi_rdma.witx"],
    });
    impl From<GuestError> for RdmaError{
        fn from(value: GuestError) -> Self {
            // match value {
            //     GuestError::InvalidFlagValue(_) => {}
            //     GuestError::InvalidEnumValue(_) => {}
            //     GuestError::PtrOverflow => {}
            //     GuestError::PtrOutOfBounds(_) => {}
            //     GuestError::PtrNotAligned(_, _) => {}
            //     GuestError::PtrBorrowed(_) => {}
            //     GuestError::BorrowCheckerOutOfHandles => {}
            //     GuestError::SliceLengthsDiffer => {}
            //     GuestError::InFunc { .. } => {}
            //     GuestError::InvalidUtf8(_) => {}
            //     GuestError::TryFromIntError(_) => {}
            // }
            return Self::RuntimeError;
        }
    }