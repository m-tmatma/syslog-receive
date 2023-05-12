use std::net::UdpSocket;
use std::str;
use std::thread;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:514")?;
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((buf_size, src_addr)) => {
                thread::spawn(move || {
                    let buf = &mut buf[..buf_size];
                    let req_msg = str::from_utf8(&buf).unwrap();
                    println!("{:}", "=".repeat(80));
                    println!("request message: {0: >8} {1} {2}", buf_size, src_addr, req_msg);
                });
            }
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }
    }
}
