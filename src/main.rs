use std::net::UdpSocket;
use std::str;
use std::thread;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:514")?;
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((buf_size, src_addr)) => {
                thread::spawn(move || {
                    let path = format!("{}.txt", src_addr.ip().to_string());
                    println!("{}", &path);

                    let mut file = File::options().append(true).open(path).unwrap();
                    let buf = &mut buf[..buf_size];
                    let req_msg = str::from_utf8(&buf).unwrap();
                    let message = format!("request message: {0: >8} {1} {2}", buf_size, src_addr, req_msg);
                    println!("{}", &message);
                    writeln!(file, "{}", &message).unwrap();
                });
            }
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }
    }
}
