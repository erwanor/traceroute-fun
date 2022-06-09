use std::borrow::BorrowMut;
use std::net::{IpAddr, Ipv4Addr, TcpStream, UdpSocket, SocketAddrV4};

use bytes::BytesMut;
use icmp;

use std::io::prelude::*;

/*
 * 1. send TCP packets with custom  TTL
 * 2. wait for ICMP TTL Exceed feedback
 * 3. parse the router that sent us the packet
 * 4. log the identity along with latency metric?
 */

fn main() {
    let localhost = Ipv4Addr::new(0,0,0,0);
    let localhost_addr = SocketAddrV4::new(localhost, 1234);
    let mut socket = UdpSocket::bind(localhost_addr).expect("failed to bind udp socket");
    let addr: SocketAddrV4 = "139.178.84.217:1235".parse().unwrap();
    socket.connect(addr).expect("failed to connect to address");
    socket.set_ttl(3);
    socket.send(&[1,2,3, 4, 5]);
}
