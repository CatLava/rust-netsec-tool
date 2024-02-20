use std::time::Duration;
use ping_rs::*;
use std::net::{IpAddr, Ipv4Addr};

mod ping_tool;

fn main() {
    println!("Hello, world!");
    // CLI tool to take in network cidr
    // need a cidr map or default to /24 
    // 1. Discover devices on network with ping
    let base_ip = Ipv4Addr::new(192, 168, 1, 1);
    // TODO this can be multi-threaded 
    for num in 1..255 {
        let test_ip = Ipv4Addr::new(base_ip.octets()[0], base_ip.octets()[1], base_ip.octets()[2], num);
        println!("{:?}", test_ip);
        ping_tool::ping_address(&test_ip.to_string())
    }
    //ping_tool::ping_address("192.168.1.1")
}

