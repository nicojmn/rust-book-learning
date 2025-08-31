mod public_ip;

#[tokio::main]
async fn main() {
    let pub_ip = public_ip::get_public_ip()
        .await
        .expect("Failed to get public IPs");

    for ip in pub_ip.iter() {
        match ip {
            public_ip::IPAddr::V4(a, b, c, d) => println!("Public IP : {}.{}.{}.{}", a, b, c, d),
            public_ip::IPAddr::V6(ipv6) => println!("Public IP : {}", ipv6),
        }
    }
}
