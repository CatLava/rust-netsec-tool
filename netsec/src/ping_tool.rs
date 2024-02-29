// use this one https://docs.rs/pinger/latest/pinger/all.html#functions
use pinger::{ping, PingResult};
use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind};

pub fn ping_address(address: &str) -> Option<String> {
    let stream = ping(address.to_string(), None).expect("Error pinging");
    let mut count = 0;
    let mut valid_addr: Option<String> = None;
    for message in stream {
        if count >5 {
            return valid_addr
        }
        match message {
            PingResult::Pong(duration, _) => {//println!("{:?} : {:?}", duration, address);
                                                valid_addr = Some(address.to_string())},
            PingResult::Timeout(_) => (),
            // Unknown lines, just ignore.
            PingResult::Unknown(line) => (),
            PingResult::PingExited(_, _) => todo!()
        }
        count +=1;
    }
    return valid_addr
}


pub fn port_scanner(address: &str, ports: Option<&Vec<u16>>){
    // Need to check if ports
    let standard_ports = vec![21,22,25,53,80,135,139,443,445,1723,3389];
    println!("Searching on address : {}", address);
    let search_ports = match ports {
        Some(ports) => ports,
        _ => &standard_ports
    };
    for port in search_ports { 
        let addr = format!("{}:{}", address, port);
        // convert addr into socket addr
        let socket_addr = match addr.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => panic!("Failed to parse addr: {}", e)
        };
        match TcpStream::connect_timeout(&socket_addr, std::time::Duration::from_secs(1)) {
            Ok(_) => println!("Port {} is open", port),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => (),//println!("Port {} is closed", port),
                _ => (),//println!("Failed to connect: {}", e),
            },
        }
    }
    return


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

