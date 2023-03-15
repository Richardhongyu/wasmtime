use std::ptr::null_mut;
use std::sync::Arc;

pub struct RDMA {
    // ...
    pub(crate) id: Arc<*mut rdma_cm_id>,
    pub(crate) ai_family: i32,
    pub(crate) ai_socktype: i32,
    pub(crate) ai_qb_type: i32,
    pub(crate) ai_port_space: i32,
}
unsafe impl Send for RDMA {}
unsafe impl Sync for RDMA {}

impl Default for RDMA {
    fn default() -> Self {
        Self {
            // ...
            id: Arc::from(null_mut()),
            // -1 means Default.
            ai_family: -1,
            ai_socktype: -1,
            ai_qb_type: -1,
            ai_port_space: -1,
        }
    }
}

}
