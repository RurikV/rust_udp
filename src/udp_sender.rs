use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args = std::env::args();
    let send_addr = args.nth(1).expect("sender address expected");
    let recv_addr = args.next().expect("receiver address expected");

    let sender = UdpSocket::bind(&send_addr).unwrap();

    for i in 0..100_000_000_000u64 {
        match sender.send_to(&i.to_le_bytes(), &recv_addr) {
            Ok(_) => {
                // add logging for every sent packet here if necessary
            }
            Err(e) => {
                eprintln!("Failed to send packet {}: {}", i, e);
            }
        }

        // introduced a small sleep to avoid overwhelming the receiver or the network.
        sleep(Duration::from_micros(10));
    }
}