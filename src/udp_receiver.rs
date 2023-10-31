use std::net::UdpSocket;

fn main() {
    let mut args = std::env::args();
    let recv_addr = args.nth(1).expect("receiver address expected");

    let receiver = UdpSocket::bind(&recv_addr).unwrap();
    
    let mut expected_number = 0u64;

    for _ in 0..100_000_000_000u64 {
        let mut buf = [0; 8];
        match receiver.recv_from(&mut buf) {
            Ok((_red, _send_addr)) => {
                let received_number = u64::from_le_bytes(buf);
                
                if received_number != expected_number {
                    println!("Missed packet! Expected: {}, Received: {}", expected_number, received_number);
                    expected_number = received_number + 1;  // Adjust the expected number based on the packet received
                } else {
                    expected_number += 1;
                }

                // println!("Received {} from {}", received_number, send_addr);
            }
            Err(e) => {
                eprintln!("Error while receiving: {}", e);
            }
        }
    }
}
