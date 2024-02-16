use std::time::Duration;
use ping_rs::*;
use std::net::{IpAddr, Ipv4Addr};

mod ping_tool;

fn main() {
    println!("Hello, world!");
    // CLI tool to take in network cidr
    // need a cidr map or default to /24 
    // 1. Discover devices on network with ping
    base_ip = Ipv4Addr(192, 168, 1, 1)
    for num in 1..255 {
        println!("{:?}", num)
    }
    ping_tool::ping_address("192.168.1.1")
}

