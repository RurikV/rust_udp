use std::net::UdpSocket;

fn main() {
    let mut args = std::env::args();
    let send_addr = args.nth(1).expect("sender address expected");
    let recv_addr = args.next().expect("receiver address expected");

    let sender = UdpSocket::bind(send_addr).unwrap();
    for i in 0..10000u32 {
        sender.send_to(&i.to_le_bytes(), &recv_addr).unwrap();
    }
}
