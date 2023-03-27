mod generated;
pub use generated::*;
impl Default for IbvQpCap{
    fn default() -> Self {
        Self {
            max_send_wr: 1,
            max_recv_wr: 1,
            max_send_sge: 1,
            max_recv_sge: 1,
            max_inline_data: 16,
        }
    }
}
impl Default for RdmaAddrinfoStruct{
    fn default() -> Self {
        Self {
            flags: 0,
            port_space: 0,
            family: 0,
            qp_type: 0,
            src_len: 0,
            dst_len: 0,
        }
    }
}
pub mod rdma_port_space {
    pub type Type = ::std::os::raw::c_uint;
    pub const RDMA_PS_IPOIB: Type = 2;
    pub const RDMA_PS_TCP: Type = 262;
    pub const RDMA_PS_UDP: Type = 273;
    pub const RDMA_PS_IB: Type = 319;
}
pub mod rdma_addrinfo_flags {
    pub type Type = ::std::os::raw::c_uint;
    pub const RAI_PASSIVE: Type = 1;
    pub const RAI_NUMERICHOST: Type = 2;
    pub const RAI_NOROUTE: Type = 4;
    pub const RAI_FAMILY: Type = 8;

}
pub mod rdma_addrinfo_family {
    pub type Type = ::std::os::raw::c_uint;
    pub const AF_INET: Type = 2;
    pub const AF_INET6: Type = 10;
    pub const AF_IB: Type = 27;
}
pub mod rdma_addrinfo_qp_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_QPT_RC: Type = 2;
    pub const IBV_QPT_UC: Type = 3;
    pub const IBV_QPT_UD: Type = 4;
    pub const IBV_QPT_RAW_PACKET: Type = 8;
    pub const IBV_QPT_XRC_SEND: Type = 9;
    pub const IBV_QPT_XRC_RECV: Type = 10;
    pub const IBV_QPT_DRIVER: Type = 255;
}
pub mod rdma_ibv_qp_attr_mask{
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_QP_STATE: Type = 1;
    pub const IBV_QP_CUR_STATE: Type = 2;
    pub const IBV_QP_EN_SQD_ASYNC_NOTIFY: Type = 4;
    pub const IBV_QP_ACCESS_FLAGS: Type = 8;
    pub const IBV_QP_PKEY_INDEX: Type = 16;
    pub const IBV_QP_PORT: Type = 32;
    pub const IBV_QP_QKEY: Type = 64;
    pub const IBV_QP_AV: Type = 128;
    pub const IBV_QP_PATH_MTU: Type = 256;
    pub const IBV_QP_TIMEOUT: Type = 512;
    pub const IBV_QP_RETRY_CNT: Type = 1024;
    pub const IBV_QP_RNR_RETRY: Type = 2048;
    pub const IBV_QP_RQ_PSN: Type = 4096;
    pub const IBV_QP_MAX_QP_RD_ATOMIC: Type = 8192;
    pub const IBV_QP_ALT_PATH: Type = 16384;
    pub const IBV_QP_MIN_RNR_TIMER: Type = 32768;
    pub const IBV_QP_SQ_PSN: Type = 65536;
    pub const IBV_QP_MAX_DEST_RD_ATOMIC: Type = 131072;
    pub const IBV_QP_PATH_MIG_STATE: Type = 262144;
    pub const IBV_QP_CAP: Type = 524288;
    pub const IBV_QP_DEST_QPN: Type = 1048576;
    pub const IBV_QP_RATE_LIMIT: Type = 33554432;
}
pub mod rdma_ibv_send_flags{
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_SEND_FENCE: Type = 1;
    pub const IBV_SEND_SIGNALED: Type = 2;
    pub const IBV_SEND_SOLICITED: Type = 4;
    pub const IBV_SEND_INLINE: Type = 8;
    pub const IBV_SEND_IP_CSUM: Type = 16;
}
