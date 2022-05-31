use std::borrow::BorrowMut;
use std::net::{IpAddr, Ipv4Addr};

use bytes::BytesMut;
use icmp;

/*
 * 1. send TCP packets with custom  TTL
 * 2. wait for ICMP TTL Exceed feedback
 * 3. parse the router that sent us the packet
 * 4. log the identity along with latency metric?
 */

fn main() {
    println!("Hello, world!");
    let mut socket = icmp::IcmpSocket::connect(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        .expect("failed to open raw socket");

    let mut b: [u8; 32] = [0; 32];

    let mut buffer = BytesMut::with_capacity(1024);
    loop {
        socket.send(&mut b).expect("failed to send over socket!");

        let bytes_read = socket
            .recv(buffer.borrow_mut())
            .expect("failed to read from socket!");
        println!("{bytes_read}");
    }
}
