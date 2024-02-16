// use this one https://docs.rs/pinger/latest/pinger/all.html#functions
use pinger::{ping, PingResult};

pub fn ping_address(address: &str) {
    let stream = ping(address.to_string(), None).expect("Error pinging");
    for message in stream {
        match message {
            PingResult::Pong(duration, _) => println!("{:?} : {:?}", duration, address),
            PingResult::Timeout(_) => println!("Timeout!"),
            // Unknown lines, just ignore.
            PingResult::Unknown(line) => (),
            PingResult::PingExited(_, _) => todo!()
        }
    }
}