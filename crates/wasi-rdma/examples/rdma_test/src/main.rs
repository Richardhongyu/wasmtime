// TODO: fix the warning when fix the error handle
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::time::{SystemTime, UNIX_EPOCH};
use wasi_rdma;
static SERVER: &str = "192.168.217.128\0";
static PORT: &str = "7471\0";

fn main() {
    // println!("RDMA Client Test");
    // wasi::thread_spawn
    // unsafe{wasi::thread_spawn(4);}
    // std::thread::spawn(move || server_runs(SERVER, PORT));
    // sleep for 1 second
    // std::thread::sleep(Duration::new(1, 0));
    // print the current time
    let now1 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    println!("the current time is {:?}", now1);
    let _ret = client_runs(SERVER, PORT);
    // print the end time
    let now2 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    println!("the end time is {:?}", now2);
    println!("the between time is {:?}", now2 - now1);
    // println!("End of rdma_test");
}

fn client_runs(_ip: &str, _port: &str) -> i32 {
    // println!("rdma_server: start");
    let ret = client_run();
    if ret != 0 {
        println!(
            "rdma_server: ret error {:?}",
            std::io::Error::from_raw_os_error(-ret)
        );
        if ret == -1 {
            println!(
                "rdma_server: last os error {:?}",
                std::io::Error::last_os_error()
            );
        }
    }
    // println!("rdma_server: end");
    0
}

// fn server_runs(_ip: &str, _port: &str) -> i32 {
//     // println!("rdma_server: start");
//     let ret = server_run();
//     if ret != 0 {
//         // println!(
//             "rdma_server: ret error {:?}",
//             std::io::Error::from_raw_os_error(-ret)
//         );
//         if ret == -1 {
//             // println!(
//                 "rdma_server: last os error {:?}",
//                 std::io::Error::last_os_error()
//             );
//         }
//     }
//     // println!("rdma_server: end");
//     0
// }

fn client_run() -> i32 {
    let mut send_flags = 0_u32;
    let mut send_msg = vec![1_u8; 32];
    let mut recv_msg = vec![0_u8; 32];

    let mut rdma_info = wasi_rdma::RdmaAddrinfoStruct::default();

    let cap = wasi_rdma::IbvQpCap::default();
    rdma_info.port_space = wasi_rdma::rdma_port_space::RDMA_PS_TCP;
    // TODO: add the send_flags support
    // if cap.max_inline_data >= 32 {
    //     send_flags = ibv_send_flags::IBV_SEND_INLINE.0;
    // } else {
    //     // println!("rdma_client: device doesn't support IBV_SEND_INLINE, using sge sends");
    // }

    let rdma = unsafe { wasi_rdma::rdma_init(SERVER, PORT, rdma_info, cap, 0).unwrap() };

    // println!("client");
    let mr = unsafe { wasi_rdma::rdma_reg_msgs(rdma, recv_msg.as_mut_ptr().cast(), 32).unwrap() };
    send_flags = unsafe { wasi_rdma::rdma_send_flags(rdma).unwrap() };
    // println!("client");
    let send_mr = if (send_flags & wasi_rdma::rdma_ibv_send_flags::IBV_SEND_INLINE) as u32 == 0 {
        println!("client send_flags:{}",send_flags);
        unsafe { wasi_rdma::rdma_reg_msgs(rdma, send_msg.as_mut_ptr().cast(), 32).unwrap() }
    } else {
        0
    };

    // println!("client");

    unsafe { wasi_rdma::rdma_post_recv(rdma, recv_msg.as_mut_ptr().cast(), 32, mr).unwrap() };

    // println!("client");
    unsafe { wasi_rdma::rdma_connect(rdma).unwrap() };

    // println!("client");
    unsafe {
        wasi_rdma::rdma_post_send(rdma, send_msg.as_mut_ptr().cast(), 32, send_mr, send_flags)
            .unwrap()
    };

    // println!("client");
    let wc = unsafe { wasi_rdma::rdma_get_send_comp(rdma, 0).unwrap() };
    let ret = 0;
    // println!("client end");
    // while ret == 0 {
    //     ret = unsafe { wasi_rdma::rdma_get_send_comp(rdma, wc).unwrap() };
    //     // println!("client ret: {}", ret);
    // }
    // println!("client");
    // TODO: fix the error handle
    // if ret < 0 {
    //     // println!("rdma_get_send_comp");
    //     unsafe {
    //         wasi_rdma::rdma_disconnect(Rdma);
    //     }
    //     return ret;
    // }

    // while ret == 0 {
    unsafe { wasi_rdma::rdma_get_recv_comp(rdma, wc).unwrap() };
    // }
    // println!("rdma_client: recv msg : {:?}", recv_msg);
    // TODO: fix the error handle
    // if ret < 0 {
    //     // println!("rdma_get_recv_comp");
    // } else {
    //     ret = 0;
    // }

    0
}

// fn server_run() -> i32 {
//     let mut send_flags = 0_u32;
//     let mut send_msg = vec![1_u8; 32];
//     let mut recv_msg = vec![0_u8; 32];

//     let rdma_info = wasi_rdma::RdmaAddrinfoStruct {
//         flags: 0,
//         port_space: 0,
//         family: 0,
//         qp_type: 0,
//         src_len: 0,
//         dst_len: 0,
//     };

//     let cap = wasi_rdma::IbvQpCap {
//         max_send_wr: 1,
//         max_recv_wr: 1,
//         max_send_sge: 1,
//         max_recv_sge: 1,
//         max_inline_data: 32,
//     };

//     let rdma = unsafe { wasi_rdma::rdma_init(SERVER, PORT, rdma_info, cap, 1).unwrap() };

//     // TODO: add the support of IBV_QP_CAP
//     let mut ret = unsafe { wasi_rdma::ibv_query_qp(rdma, 1 << 19).unwrap() };

//     send_flags = 8;
//     // TODO: add the send_flags support
//     // if cap.max_inline_data >= 32 {
//     //     send_flags = ibv_send_flags::IBV_SEND_INLINE.0;
//     // } else {
//     //     // println!("rdma_server: device doesn't support IBV_SEND_INLINE, using sge sends");
//     // }

//     let mr = unsafe { wasi_rdma::rdma_reg_msgs(rdma, recv_msg.as_mut_ptr().cast(), 32).unwrap() };

//     let send_mr =
//         unsafe { wasi_rdma::rdma_reg_msgs(rdma, send_msg.as_mut_ptr().cast(), 32).unwrap() };

//     ret = unsafe { wasi_rdma::rdma_post_recv(rdma, recv_msg.as_mut_ptr().cast(), 32, mr).unwrap() };

//     ret = unsafe { wasi_rdma::rdma_accept(rdma).unwrap() };
//     // TODO: fix the error handle
//     // if ret != 0 {
//     //     // println!("rdma_accept");
//     //     // TODO: add the send_flags support
//     //     if (send_flags & 8) == 0 {
//     //         unsafe { wasi_rdma::rdma_dereg_mr(Send_Mr) };
//     //     }
//     //     return ret;
//     // }

//     let wc = unsafe { wasi_rdma::rdma_get_send_comp(rdma, 0).unwrap() };
//     let mut ret = 0;
//     while ret == 0 {
//         ret = unsafe { wasi_rdma::rdma_get_recv_comp(rdma, wc).unwrap() };
//     }
//     // println!("rdma_client: recv msg : {:?}", recv_msg);
//     // TODO: fix the error handle
//     // if ret < 0 {
//     //     // println!("rdma_get_recv_comp");
//     // } else {
//     //     ret = 0;
//     // }

//     let _rets = unsafe {
//         wasi_rdma::rdma_post_send(rdma, send_msg.as_mut_ptr().cast(), 32, send_mr, send_flags)
//             .unwrap()
//     };

//     let mut ret = 0;

//     ret = unsafe { wasi_rdma::rdma_get_send_comp(rdma, wc).unwrap() };

//     // TODO: fix the error handle
//     // if ret < 0 {
//     //     // println!("rdma_get_send_comp");
//     //     unsafe {
//     //         wasi_rdma::rdma_disconnect(Rdma);
//     //     }
//     //     return ret;
//     // }

//     0
// }
