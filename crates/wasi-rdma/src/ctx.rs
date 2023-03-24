use rdma_sys::*;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::sync::Arc;

use wiggle::GuestPtr;

use crate::guest_types::IbvWc;
use crate::guest_types::RdmaError::RuntimeError;
use crate::guest_types::{IbvMr, IbvQpCap, Rdma, RdmaAddrinfoStruct, RdmaError};
use crate::rdma::{RdmaIbvWc, RdmaMr, RDMA};
use crate::table;
use crate::witx::wasi_ephemeral_rdma::WasiEphemeralRdma;

pub struct WasiRdmaCtx {
    table: table::Table,
}

impl WasiRdmaCtx {
    pub fn new() -> Self {
        Self {
            table: table::Table::new(),
        }
    }
}

impl WasiEphemeralRdma for WasiRdmaCtx {
    fn rdma_init<'a>(
        &mut self,
        node: &wiggle::GuestPtr<'a, str>,
        service: &wiggle::GuestPtr<'a, str>,
        hints: &RdmaAddrinfoStruct,
        cap: &IbvQpCap,
        is_server: u8,
    ) -> Result<Rdma, RdmaError> {
        let mut hint: rdma_addrinfo = hints.into();
        let mut info: *mut rdma_addrinfo = null_mut();
        hint.ai_port_space = rdma_port_space::RDMA_PS_TCP as c_int;
        // Safety: ffi
        let node = node.as_str()?.ok_or(RdmaError::RuntimeError)?;
        let service = service.as_str()?.ok_or(RdmaError::RuntimeError)?;
        let mut ret = unsafe {
            rdma_getaddrinfo(
                node.as_ptr().cast(),
                service.as_ptr().cast(),
                &hint,
                &mut info,
            )
        };
        if ret != 0_i32 {
            return Err(RdmaError::RuntimeError);
        }
        let mut id: *mut rdma_cm_id = null_mut();
        // Safety: ffi
        let mut init_attr = unsafe { std::mem::zeroed::<ibv_qp_init_attr>() };
        init_attr.cap = cap.into();
        //todo: qp_context?
        // init_attr.qp_context = id.cast();
        init_attr.sq_sig_all = 1;
        ret = unsafe { rdma_create_ep(&mut id, info, null_mut(), &mut init_attr) };
        if ret != 0 {
            // Safety: ffi
            unsafe {
                rdma_freeaddrinfo(info);
            }
            return Err(RuntimeError);
        }

        // Safety: id was initialized by `rdma_create_ep`
        let mut rdma = RDMA::default();
        if is_server != 0 {
            rdma.is_server = true;
            ret = unsafe { rdma_listen(id, 0) };
            rdma.listen_id = id;
            if ret != 0 {
                unsafe {
                    rdma_destroy_ep(id);
                }
                return Err(RuntimeError);
            }
            let mut _listen_id: *mut rdma_cm_id = null_mut();
            ret = unsafe { rdma_get_request(id, &mut _listen_id) };
            if ret != 0 {
                unsafe {
                    rdma_destroy_ep(id);
                }
                return Err(RuntimeError);
            }
            rdma.id = _listen_id;
        } else {
            rdma.id = id;
        }
        rdma.init_attr = init_attr;

        // Safety: ffi

        Ok(self
            .table
            .push(Arc::new(rdma))
            .map_err(|_| RuntimeError)?
            .into())
    }

    fn rdma_connect(&mut self, rdma: Rdma) -> Result<(), RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;
        if rdma.is_server {
            println!("RDMA Server ->rdma_connect ?");
        }

        let id = rdma.id()?;
        let ret = unsafe { rdma_connect(id, null_mut()) };
        if ret != 0 {
            unsafe {
                rdma_disconnect(id);
            }
            return Err(RuntimeError);
        }
        Ok(())
    }

    fn rdma_disconnect(&mut self, rdma: Rdma) -> Result<(), RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;

        let id = rdma.id()?;
        unsafe { rdma_disconnect(id) };
        Ok(())
    }
    fn rdma_accept(&mut self, rdma: Rdma) -> Result<(), RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;
        let id = rdma.id()?;
        let ret = unsafe { rdma_accept(id, null_mut()) };
        if ret != 0 {
            return Err(RuntimeError);
        }
        Ok(())
    }
    fn rdma_send_flags(&mut self, rdma: Rdma) -> Result<u32, RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;
        Ok(rdma.send_flags)
    }

    fn rdma_get_send_comp(&mut self, rdma: Rdma, wc: IbvWc) -> Result<IbvWc, RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;
        let id = rdma.id()?;
        //if ibv_wc is NULL,set to 0;
        let ibv_wc_: *mut ibv_wc = if wc == 0.into() {
            unsafe { null_mut() }
        } else {
            self.table
                .get_mut::<RdmaIbvWc>(wc.into())
                .map_err(|_| RuntimeError)?
                .0
        };
        let ret = unsafe { rdma_get_send_comp(id, ibv_wc_) };
        if ret != 0 {
            return Err(RuntimeError);
        }
        if wc == 0.into() {
            Ok(self
                .table
                .push(Arc::new(RdmaIbvWc(ibv_wc_)))
                .map_err(|_| RuntimeError)?
                .into())
        } else {
            Ok(wc)
        }
    }

    fn rdma_get_recv_comp(&mut self, rdma: Rdma, wc: IbvWc) -> Result<IbvWc, RdmaError> {
        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;
        let id = rdma.id()?;
        //if ibv_wc is NULL,set to 0;
        let ibv_wc_: *mut ibv_wc = if wc == 0.into() {
            unsafe { null_mut() }
        } else {
            self.table
                .get_mut::<RdmaIbvWc>(wc.into())
                .map_err(|_| RuntimeError)?
                .0
        };
        let ret = unsafe { rdma_get_recv_comp(id, ibv_wc_) };
        if ret != 0 {
            return Err(RuntimeError);
        }
        if wc == 0.into() {
            Ok(self
                .table
                .push(Arc::new(RdmaIbvWc(ibv_wc_)))
                .map_err(|_| RuntimeError)?
                .into())
        } else {
            Ok(wc)
        }
    }

    fn rdma_reg_msgs<'a>(
        &mut self,
        rdma: Rdma,
        addr: &GuestPtr<'a, u8>,
        size: u32,
    ) -> Result<IbvMr, RdmaError> {
        // TODO: Check Memory
        if addr.is_shared_memory() {
            println!("No Support for Shared Memory!");
            return Err(RuntimeError);
        }
        let mut addr = addr
            .as_array(size)
            .as_slice_mut()
            .map_err(|_| RuntimeError)?.ok_or_else(|| RuntimeError)?;

        let rdma: Arc<RDMA> = self.table.get(rdma.into()).map_err(|_| RuntimeError)?;

        let id = rdma.id()?;
        let mr = unsafe { rdma_reg_msgs(id, addr.as_mut_ptr().cast(), size as usize) };
        if mr.is_null() {
            unsafe { rdma_dereg_mr(mr) };
            return Err(RuntimeError);
        }
        Ok(self
            .table
            .push(Arc::new(RdmaMr(mr)))
            .map_err(|_| RuntimeError)?
            .into())
    }

    fn rdma_dereg_mr(&mut self, ibv_mr: IbvMr) {
        let mr = self
            .table
            .get_mut::<RdmaMr>(ibv_mr.into())
            .map_err(|_| RuntimeError);
        if let Ok(inner_mr) = mr {
            unsafe { rdma_dereg_mr(inner_mr.0) };
        }
    }

    fn rdma_post_send<'a>(
        &mut self,
        rdma: Rdma,
        addr: &GuestPtr<'a, u8>,
        _size: u32,
        send_mr: IbvMr,
        flags: u32,
    ) -> Result<(), RdmaError> {
        let rdma: &mut RDMA = self
            .table
            .get_mut::<RDMA>(rdma.into())
            .map_err(|_| RuntimeError)?;
        let id = rdma.id()?;
        let send_msg = unsafe { addr.mem().base().as_ptr().offset(addr.offset() as isize) };
        let send_mr = self
            .table
            .get_mut::<RdmaMr>(send_mr.into())
            .map_err(|_| RuntimeError)?
            .0;

        let ret = unsafe {
            rdma_post_send(
                id,
                null_mut(),
                send_msg as *mut c_void,
                16,
                send_mr,
                flags as i32,
            )
        };
        if ret != 0 {
            println!("rdma_post_send");
            unsafe {
                rdma_disconnect(id);
            }
            return Err(RuntimeError);
        }
        Ok(())
    }

    fn rdma_post_recv<'a>(
        &mut self,
        rdma: Rdma,
        addr: &GuestPtr<'a, u8>,
        _size: u32,
        recv_mr: IbvMr,
    ) -> Result<(), RdmaError> {
        let rdma = self
            .table
            .get_mut::<RDMA>(rdma.into())
            .map_err(|_| RuntimeError)?;
        let id = rdma.id()?;
        let send_flags = rdma.send_flags;
        let recv_msg = unsafe { addr.mem().base().as_ptr().offset(addr.offset() as isize) };
        let mr = self
            .table
            .get_mut::<RdmaMr>(recv_mr.into())
            .map_err(|_| RuntimeError)?
            .0;
        let ret = unsafe { rdma_post_recv(id, null_mut(), recv_msg as *mut c_void, 16, mr) };
        if ret != 0 {
            println!("rdma_post_recv");
            if (send_flags & ibv_send_flags::IBV_SEND_INLINE.0) as u32 == 0 {
                unsafe { rdma_dereg_mr(mr) };
            }
            return Err(RuntimeError);
        }
        Ok(())
    }

    fn ibv_query_qp(&mut self, rdma: Rdma, ibv_qp_attrmask: u32) -> Result<(), RdmaError> {
        let rdma: &mut RDMA = self
            .table
            .get_mut::<RDMA>(rdma.into())
            .map_err(|_| RuntimeError)?;
        let mut qp_attr = unsafe { std::mem::zeroed::<ibv_qp_attr>() };
        let id = rdma.id()?;
        // let mask:c_int = if ibv_qp_attrmask>0{ibv_qp_attr_mask(ibv_qp_attrmask).0.try_into().unwrap_or(ibv_qp_attr_mask::IBV_QP_CAP.0.try_into().unwrap())}else {ibv_qp_attr_mask::IBV_QP_CAP.0.try_into().unwrap()};
        let mask: c_int = if ibv_qp_attrmask > 0 {
            ibv_qp_attrmask
        } else {
            ibv_qp_attr_mask::IBV_QP_CAP.0.try_into().unwrap()
        } as c_int;
        let ret = unsafe { ibv_query_qp((*id).qp, &mut qp_attr, mask, &mut rdma.init_attr) };
        if ret != 0 {
            println!("ibv_query_qp error");
            unsafe {
                rdma_destroy_ep(id);
            }
            return Err(RuntimeError);
        }
        if rdma.init_attr.cap.max_inline_data >= 16 {
            rdma.send_flags = ibv_send_flags::IBV_SEND_INLINE.0;
        } else {
            println!("rdma_server: device doesn't support IBV_SEND_INLINE, using sge sends");
        }
        Ok(())
    }

    fn print_hello_world(&mut self) {
        println!("Hello World!");
    }
}
