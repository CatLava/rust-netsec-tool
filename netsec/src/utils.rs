use regex::Regex;

pub fn validate_str_to_ip(ip_str: String) -> Option<Vec<u8>>{
    let re = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").unwrap();
    // This will need to be a match 
    if let Some(cap) = re.captures(&ip_str) {
        let octet1 = cap[1].parse::<u8>().unwrap();
        let octet2 = cap[2].parse::<u8>().unwrap();
        let octet3 = cap[3].parse::<u8>().unwrap();
        let octet4 = cap[4].parse::<u8>().unwrap();
        octet1 <= 255 && octet2 <= 255 && octet3 <= 255 && octet4 <= 255;
        return Some([octet1, octet2, octet3, octet4].to_vec())
    } else{
        return None;
    }
    // Need to return a list at the end 
}
