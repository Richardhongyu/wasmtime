use std::ptr::null_mut;
use std::sync::Arc;
use rdma_sys::{ibv_qp_cap, ibv_qp_init_attr, rdma_addr, rdma_addrinfo, rdma_cm_id};
use crate::wasi_rdma_wasmtime::types::{IbvQpCap, RdmaAddrinfoStruct, RdmaError};
use crate::wasi_rdma_wasmtime::types::RdmaError::RuntimeError;
pub struct RDMA {
    // ...
    pub(crate) id:  *mut rdma_cm_id,
    pub(crate) listen_id: *mut rdma_cm_id,
    pub is_server: bool,
    pub init_attr: ibv_qp_init_attr,
}
impl RDMA{
    pub(crate) fn id(&self) ->Result<*mut rdma_cm_id,RdmaError>{
        if self.id.is_null() {
            println!("Null Context!");
            Err(RuntimeError)
        }else { Ok(self.id) }
    }
}
unsafe impl Send for RDMA {}
unsafe impl Sync for RDMA {}

pub struct RdmaMr( pub *mut rdma_sys::ibv_mr);
unsafe impl Send for RdmaMr {}
unsafe impl Sync for RdmaMr {}


impl Default for RDMA {
    fn default() -> Self {
        Self {
            // ...
            id:  null_mut(),
            listen_id:  null_mut(),
            is_server: false,
            init_attr: unsafe { std::mem::zeroed::<ibv_qp_init_attr>() },
        }
    }
}



impl From<&RdmaAddrinfoStruct> for rdma_addrinfo {
    fn from(s: &RdmaAddrinfoStruct) -> rdma_addrinfo {
        let mut addrinfo:rdma_addrinfo=unsafe { std::mem::zeroed::<rdma_addrinfo>() };
        if s.flags>0 {
            addrinfo.ai_flags=s.flags;
        }
        if s.family>0 {
            addrinfo.ai_family=s.family;
        }
        if s.qp_type>0 {
            addrinfo.ai_qp_type=s.qp_type;
        }
        if s.port_space>0 {
            addrinfo.ai_port_space=s.port_space;
        }
        if s.src_len>0 {
            addrinfo.ai_src_len=s.src_len;
        }
        if s.dst_len>0 {
            addrinfo.ai_dst_len=s.dst_len;
        }

        addrinfo
        
    }
}
impl From<&IbvQpCap> for ibv_qp_cap {
    fn from(s: &IbvQpCap) -> ibv_qp_cap {
        let mut cap:ibv_qp_cap=unsafe { std::mem::zeroed::<ibv_qp_cap>() };
        cap.max_inline_data = if s.max_inline_data>0 { s.max_inline_data} else { 16 };
        cap.max_recv_sge = if s.max_recv_sge>0 { s.max_recv_sge} else { 1 };
        cap.max_recv_wr = if s.max_recv_wr>0 { s.max_recv_wr} else { 1 };
        cap.max_send_sge = if s.max_send_sge>0 { s.max_send_sge} else { 1 };
        cap.max_send_wr = if s.max_send_wr>0 { s.max_send_wr} else { 1 };
        cap
    }
}


