use std::time::Duration;
use ping_rs::*;
use std::net::IpAddr;


fn main() {
    println!("Hello, world!");
    // CLI tool to take in network cidr
    // need a cidr map or default to /24 
    // 1. Discover devices on network with ping
    ping_requsts("test".to_string())
}

// This ping request does not work, try another one 
pub fn ping_requsts(ip: String) {
    let addr: IpAddr = "192.168.1.1".parse().unwrap();
    let data = [8; 8];  // ping data
    let timeout = Duration::from_secs(10);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
    println!("{:?}", result);
    match result {
        Ok(reply) => println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl),
        Err(e) => println!("{:?}", e)
    }
}
