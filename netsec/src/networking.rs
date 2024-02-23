use pnet::datalink::{interfaces, NetworkInterface};

pub fn list_interfaces() {
    let interfaces = interfaces();
    for interface in interfaces.iter() {
        println!("This is the name {}", interface.name);
        println!("  MAC Address: {:?}", interface.mac.unwrap());
        println!("  IP Address: {}", interface.ips.iter().map(|ip| ip.to_string()).collect::<Vec<_>>().join(", "));
    }

}