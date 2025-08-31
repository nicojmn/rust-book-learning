use std::collections::HashMap;

pub enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub async fn get_public_ip() -> Result<Vec<IPAddr>, Box<dyn std::error::Error>> {
    let ipv4 = reqwest::get("https://api.ipify.org/?format=json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let ipv4: IPAddr = match ipv4.get("ip") {
        Some(ip) => {
            let mut bytes: Vec<u8> = Vec::new();
            for byte in ip.split('.') {
                match byte.parse::<u8>() {
                    Ok(byte) => bytes.push(byte),
                    Err(_) => return Err(format!("Invalid byte : {}", byte).into()),
                };
            }
            IPAddr::V4(bytes[0], bytes[1], bytes[2], bytes[3])
        }
        None => return Err(format!("No public IPv4").into()),
    };

    let ipv6 = reqwest::get("https://api6.ipify.org/?format=json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let ipv6: String = match ipv6.get("ip") {
        Some(ip) => String::from(ip),
        None => format!("No public IPv6"),
    };

    Ok(vec![ipv4, IPAddr::V6(ipv6)])
}
