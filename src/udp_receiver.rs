use std::net::UdpSocket;

fn main() {
    let mut args = std::env::args();
    let recv_addr = args.nth(1).expect("receiver address expected");

    let receiver = UdpSocket::bind(&recv_addr).unwrap();
    let mut received_packets: Vec<[u8; 1024]> = Vec::new();

    let mut expected_number = 0u64;

    for _ in 0..100_000_000_000u64 {
        let mut buf = [0; 1024];
        match receiver.recv_from(&mut buf) {
            Ok((bytes_received, _send_addr)) => {

                if bytes_received != 1024 {
                    eprintln!("Warning: Received {} bytes instead of 1024 bytes!", bytes_received);
                    continue; // Skip processing for this packet
                }

                let num = [buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7] ];
                let received_number = u64::from_le_bytes(num);
                
                if received_number != expected_number {
                    println!("Missed packet! Expected: {}, Received: {}", expected_number, received_number);
                    expected_number = received_number + 1;  // Adjust the expected number based on the packet received
                } else {
                    expected_number += 1;
                }

                // println!("Received {}", received_number);
                received_packets.push(buf);
            }
            Err(e) => {
                eprintln!("Error while receiving: {}", e);
            }
        }
    }
}
