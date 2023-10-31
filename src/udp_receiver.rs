use std::net::UdpSocket;
use std::time::Duration;

fn main() {
    let mut args = std::env::args();
    let recv_addr = args.nth(1).expect("sender address expected");

    let sender = UdpSocket::bind(recv_addr).unwrap();
    for _ in 0..10000u32 {
        std::thread::sleep(Duration::from_secs_f32(0.1));
        let mut buf = [0; 2];
        let (red, send_addr) = sender.recv_from(&mut buf).unwrap();
        println!("Received {red} bytes");

        let num = [buf[0], buf[1], 0, 0];
        let number = u32::from_le_bytes(num);
        println!("Received {number} from {send_addr}");
    }
}
