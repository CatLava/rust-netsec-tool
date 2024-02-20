// use this one https://docs.rs/pinger/latest/pinger/all.html#functions
use pinger::{ping, PingResult};

pub fn ping_address(address: &str) {
    let stream = ping(address.to_string(), None).expect("Error pinging");
    let mut count = 0;
    for message in stream {
        if count >5 {
            return 
        }
        match message {
            PingResult::Pong(duration, _) => println!("{:?} : {:?}", duration, address),
            PingResult::Timeout(_) => println!("Timeout!"),
            // Unknown lines, just ignore.
            PingResult::Unknown(line) => (),
            PingResult::PingExited(_, _) => todo!()
        }
        count +=1;
    }
}


pub fn port_scanner(ports: &vec[u16]){
    return
    use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind};

// fn main() {
//     let target = "127.0.0.1";
//     let port = 80;
//     let addr = format!("{}:{}", target, port);
//     let socket_addr = match addr.parse::<SocketAddr>() {
//         Ok(addr) => addr,
//         Err(e) => panic!("Failed to parse address: {}", e),
//     };

//     match TcpStream::connect_timeout(&socket_addr, std::time::Duration::from_secs(1)) {
//         Ok(_) => println!("Port {} is open", port),
//         Err(e) => match e.kind() {
//             ErrorKind::TimedOut => println!("Port {} is closed", port),
//             _ => println!("Failed to connect: {}", e),
//         },
//     }
// }
}

