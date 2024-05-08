use std::{collections::BTreeMap, net::{Ipv4Addr, Ipv6Addr}};

use anyhow::{Context, Result};
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Serialize)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    DNS(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Peer {
    #[serde(default)]
    #[serde(rename = "peer id")]
    pub peer_id: Option<String>,
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Peers {
  ByteBuf(ByteBuf),
  PeerStruct(Vec<Peer>)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerAnnounceResponse {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerScrapeFile {
  complete: i64,
  downloaded: i64,
  incomplete: i64,
  #[serde(default)]
  name: Option<String>
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerScrapeResponse {
  files: BTreeMap<ByteBuf, TrackerScrapeFile>,

  #[serde(default)]
  #[serde(rename = "failure reason")]
  failure_reason: Option<String>,

  #[serde(default)]
  #[serde(rename = "warning message")]
  warning_message: Option<String>,
}

/**
 * Fetches the announce response from the tracker as buffer.
 * announce url example sintel.torrent: http://tracker.opentrackr.org:1337/scrape?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10
 */
pub async fn fetch_scrape_buffer(url: &str) -> Result<Vec<u8>> {
  let resp: Vec<u8> = reqwest::get(url)
    .await?
    .bytes()
    .await?
    .to_vec();

  Ok(resp)
}

pub fn fetch_scrape_buffer_to_struct(response: Vec<u8>) -> Result<TrackerScrapeResponse> {
  let response_struct = serde_bencode::from_bytes(&response)
    .with_context(|| {
      let str_resp = String::from_utf8_lossy(&response);
      format!("Bad tracker response {}", str_resp)
    });
  
  Ok(response_struct?)
}

pub async fn fetch_scrape(url: &str) -> Result<TrackerScrapeResponse> {
  let resp: Vec<u8> =
    fetch_scrape_buffer(url)
    .await
    .context("Failed to fetch scrape")?;
  fetch_scrape_buffer_to_struct(resp)
}

/**
 * Fetches the announce response from the tracker as buffer.
 * announce url example sintel.torrent: http://tracker.opentrackr.org:1337/announce?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10
 */
pub async fn fetch_announce_buffer(url: &str) -> Result<Vec<u8>> {
  let resp: Vec<u8> = reqwest::get(url)
    .await?
    .bytes()
    .await?
    .to_vec();

  Ok(resp)
}

pub fn fetch_announce_buffer_to_struct(response: Vec<u8>) -> Result<TrackerAnnounceResponse> {
  let response_struct = serde_bencode::from_bytes(&response)
    .with_context(|| {
      let str_resp = String::from_utf8_lossy(&response);
      format!("Bad tracker response {}", str_resp)
    });
  
  let normalized_response = {
    let mut response_struct: TrackerAnnounceResponse = response_struct?;
    match response_struct.peers {
      Peers::ByteBuf(peers) => {
        let peers =
          peers
            .to_vec()
            .chunks(6)
            .map(|chunk| {
              let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
              let port  = ((chunk[4] as u16) << 8) | chunk[5] as u16;
              Peer {
                peer_id: None,
                ip: IpAddr::V4(ip),
                port
              }
            })
            .collect();
        response_struct.peers = Peers::PeerStruct(peers);
      }
      Peers::PeerStruct(_) => {}
    }
    response_struct
  };

  Ok(normalized_response)
}


pub async fn fetch_announce(url: &str) -> Result<TrackerAnnounceResponse> {
  let resp: Vec<u8> =
    fetch_announce_buffer(url)
    .await
    .context("Failed to fetch announce")?;
  fetch_announce_buffer_to_struct(resp)
}
