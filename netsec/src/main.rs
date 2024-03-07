use std::time::Duration;
use ping_rs::*;
use std::net::{IpAddr, Ipv4Addr};
use rayon::prelude::*;
use clap::Parser;


mod cli;
mod networking;
mod ping_tool;
mod utils;

fn main() {
    println!("Hello, world!");
    // CLI tool to take in network cidr
    // need a cidr map or default to /24 
    // 1. Discover devices on network with ping
    networking::list_interfaces();
    let args: cli::NetInputs  = cli::NetInputs::parse();
    let mut base_ip: Option<Vec<u8>> = utils::validate_str_to_ip(args.ip.unwrap());
    println!("Ip {:?}", base_ip);

    println!("Scanning {:?} for /24 network", &base_ip);
    let mut valid_ips: Vec<String> = (1..254)
        .into_par_iter()
        .map(|x| {
        let test_ip = Ipv4Addr::new(base_ip[0], base_ip[1], base_ip[2], x);
        // println!("{:?}", test_ip);
        let ip = ping_tool::ping_address(&test_ip.to_string());
        ip
            }
        )
        .filter_map(|x| x)
        .collect::<Vec<String>>();
    println!("found IPs {:?},", {&valid_ips});
    for ip in &valid_ips {
        ping_tool::port_scanner(&ip, None);
    }
}

