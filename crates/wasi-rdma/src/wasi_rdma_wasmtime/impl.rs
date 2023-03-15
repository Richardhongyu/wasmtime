mod table;
mod rdma;

use crate::RdmaError::RuntimeError;
use std::ptr::null_mut;
use std::sync::Arc;
use rdma_sys::rdma_addrinfo;
use wiggle::{from_witx, GuestError};
use crate::rdma::RDMA;
use crate::wasi_rdma::WasiRdma;
use crate::types::{RdmaError, RdmaAddrinfo, RdmaAddrinfoStruct, Id, EpPd, IbvQpInitAttr, IbvMr, Context, IbvWc, ConnParam};
struct  RdmaCtx {
    table:table::Table,
}
from_witx!({
        witx: ["/home/lx/untitled4/ttlkPs2BIB/src/wasi_rdma.witx"],
        });
impl RdmaCtx {
    pub fn new() -> Self {
        Self {
            table: table::Table::new(),
        }
    }
}

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
        return self::RuntimeError;
    }
}
impl WasiRdma for RdmaCtx {
    fn rdma_init<'a>(
        &mut self,
        node: &wiggle::GuestPtr<'a, str>,
        service: &wiggle::GuestPtr<'a, str>,
        hints: &RdmaAddrinfoStruct,
    ) -> Result<Rdma, RdmaError>{
        let mut hints = unsafe { std::mem::zeroed::<rdma_addrinfo>() };
        let mut info: *mut rdma_addrinfo = null_mut();
        hints.ai_port_space = rdma_port_space::RDMA_PS_TCP.cast();
        //TODO: hint from user??
        // Safety: ffi
        let mut ret = unsafe {
            rdma_getaddrinfo(
                node.as_ptr().cast(),
                service.as_ptr().cast(),
                &hints,
                &mut info,
            )
        };
        if ret != 0_i32 {
            return Err(RdmaError::RuntimeError);
        }

        let mut id: *mut rdma_cm_id = null_mut();
        // Safety: ffi
        ret = unsafe { rdma_create_ep(&mut id, info, rdma.pd.as_ptr(), null_mut()) };
        if ret != 0_i32 {
            // Safety: ffi
            unsafe {
                rdma_freeaddrinfo(info);
            }
            return Err(RdmaError::RuntimeError);
        }

        let mut rdma= RDMA::default();
        rdma.id = Arc::new(id);
        // Safety: ffi
        // ret = unsafe { rdma_connect(id, null_mut()) };
        // if ret != 0_i32 {
        //     // Safety: ffi
        //     unsafe {
        //         let _ = rdma_disconnect(id);
        //     }
        //     return Err(RdmaError::RuntimeError);
        // }
        Ok(self.table.push(Arc::new(rdma))?)

    }
    fn rdma_disconnect(&mut self, id: Id) -> ();
    fn rdma_post_send<'a>(
        &mut self,
        rdma: Rdma,
        context: Context,
        addr: &wiggle::GuestPtr<'a, u8>,
        send_mr: IbvMr,
        flags: u32,
    ) -> Result<(), RdmaError>{

    }
    fn rdma_post_recv<'a>(
        &mut self,
        rdma: Rdma,
        addr: &wiggle::GuestPtr<'a, u8>,
        recv_mr: IbvMr,
    ) -> Result<(), RdmaError>{

    }
    fn rdma_get_send_comp(
        &mut self,
        rdma: Rdma,
        wc: IbvWc,
    ) -> Result<IbvWc, RdmaError>{

    }
    fn rdma_get_recv_comp(
        &mut self,
        rdma: Rdma,
        wc: IbvWc,
    ) -> Result<IbvWc, RdmaError>{

    }
}
impl wiggle::GuestErrorType for types::RdmaError {
    fn success() -> Self {
        Self::Success
    }
}
fn main() {}