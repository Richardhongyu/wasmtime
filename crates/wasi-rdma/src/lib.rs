use async_rdma::{LocalMrReadAccess, LocalMrWriteAccess, RdmaBuilder};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn ibv_get_device_list() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_ibv_get_device_list() {
        let result = ibv_get_device_list();
        assert_eq!(result, 0);
    }
}
