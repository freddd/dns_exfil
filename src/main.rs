use std::net::UdpSocket;

use dns_parser::Packet;

fn main() -> std::io::Result<()>{
    let bind_addr = "0.0.0.0:53";

    let socket = UdpSocket::bind(bind_addr)?;
 
    loop {
        let mut buf = [0; 2048];
        let (_amt, _src) = socket.recv_from(&mut buf)?;
        match Packet::parse(&buf) {
            Ok(p) => {
                p.questions
                .into_iter()
                .for_each(|q|
                    println!("{}", q.qname
                        .to_string()
                        .split('.')
                        .collect::<Vec<&str>>()
                        .first()
                        .unwrap()
                ))            
            }
            Err(e) => {
                println!("error parsing packet: {:#?}", e)
            }
        }
    }
}
