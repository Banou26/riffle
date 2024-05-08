use anyhow::{Context, Result};
use serde_bytes::ByteBuf;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Deserialize, Serialize)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    DNS(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PeerStruct {
    #[serde(rename = "peer id")]
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Peers {
  ByteBuf(ByteBuf),
  PeerStruct(Vec<PeerStruct>)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerResponse {
    pub complete: i64,
    #[serde(default)]
    pub downloaded: Option<i64>,
    pub incomplete: i64,
    pub interval: i64,
    pub peers: Peers,
    #[serde(default)]
    #[serde(rename = "min interval")]
    pub min_interval: Option<i64>,
    #[serde(default)]
    #[serde(rename = "failure reason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "tracker id")]
    #[serde(default)]
    pub tracker_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "warning message")]
    pub warning_message: Option<String>
}

// announce url example sintel.torrent
// http://tracker.opentrackr.org:1337/announce?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10

pub async fn announce_buffer(url: &str) -> Result<Vec<u8>> {
  let resp = reqwest::get(url)
    .await?
    .bytes()
    .await?
    .to_vec();

  Ok(resp)
}

pub fn announce_response_to_struct(response: Vec<u8>) -> Result<TrackerResponse> {
  serde_bencode::from_bytes(&response)
    .with_context(|| {
      let str_resp = String::from_utf8_lossy(&response);
      format!("Bad tracker response {}", str_resp)
    })
}