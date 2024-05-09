use std::{fs::File, io::Read, net::{Ipv4Addr, Ipv6Addr}};

use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    DNS(String),
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
