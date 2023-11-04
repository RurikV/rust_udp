use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args = std::env::args();
    let send_addr = args.nth(1).expect("sender address expected");
    let recv_addr = args.next().expect("receiver address expected");

    let sender = UdpSocket::bind(&send_addr).unwrap();

    for i in 0..100_000_000_000u64 {
        let mut packet = [0; 1024];
        packet[0..8].copy_from_slice(&i.to_le_bytes());

        match sender.send_to(&packet[..], &recv_addr){
            Ok(bytes_sent) => {
                if bytes_sent != 1024 {
                    eprintln!("Warning: Sent {} bytes instead of 1024 bytes!", bytes_sent);
                }
            }
            Err(e) => {
                eprintln!("Failed to send packet {}: {}", i, e);
            }
        }

        // introduced a small sleep to avoid overwhelming the receiver or the network.
        sleep(Duration::from_micros(10));
    }
}