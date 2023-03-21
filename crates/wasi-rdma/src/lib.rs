mod wasi_rdma_wasmtime;

use async_rdma::{LocalMrReadAccess, LocalMrWriteAccess, RdmaBuilder};
use portpicker::pick_unused_port;
use std::{
    alloc::Layout,
    io::{self, Write},
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

async fn client(addr: SocketAddrV4) -> io::Result<()> {
    let layout = Layout::new::<[u8; 8]>();
    let rdma = RdmaBuilder::default().connect(addr).await?;
    // alloc 8 bytes remote memory
    let mut rmr = rdma.request_remote_mr(layout).await?;
    // alloc 8 bytes local memory
    let mut lmr = rdma.alloc_local_mr(layout)?;
    // write data into lmr
    let _num = lmr.as_mut_slice().write(&[1_u8; 8])?;
    // write the second half of the data in lmr to the rmr
    rdma.write(&lmr.get(4..8).unwrap(), &mut rmr.get_mut(4..8).unwrap())
        .await?;
    // send rmr's meta data to the remote end
    rdma.send_remote_mr(rmr).await?;
    Ok(())
}

#[tokio::main]
async fn server(addr: SocketAddrV4) -> io::Result<()> {
    let rdma = RdmaBuilder::default().listen(addr).await?;
    // receive mr's meta data from client
    let lmr = rdma.receive_local_mr().await?;
    let data = *lmr.as_slice();
    println!("Data written by the client using RDMA WRITE: {:?}", data);
    assert_eq!(data, [[0_u8; 4], [1_u8; 4]].concat());
    Ok(())
}

#[tokio::main]
async fn test() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), pick_unused_port().unwrap());
    std::thread::spawn(move || server(addr));
    tokio::time::sleep(Duration::new(1, 0)).await;
    client(addr)
        .await
        .map_err(|err| println!("{}", err))
        .unwrap();
}

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

    #[test]
    fn test_rdma() {
        test();
    }
}
