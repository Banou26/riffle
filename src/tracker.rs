use std::{collections::BTreeMap, net::Ipv4Addr};

use futures::{future::{join_all, try_join_all}, TryFutureExt};
use anyhow::{Context, Result, Error};
use serde_bytes::ByteBuf;
use urlencoding::encode_binary;

use crate::{meta_info::{info_hash_buffer, info_hash_hex, tracker_urls}, utils::IpAddr};

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerPeer {
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
  PeerStruct(Vec<TrackerPeer>)
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
  #[serde(default)]
  downloaded: Option<i64>,
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


#[derive(Debug)]
pub struct TrackerScrapeNormalized {
  pub url: String,

  pub files: BTreeMap<ByteBuf, TrackerScrapeFile>,
  pub failure_reason: Option<String>,
  pub warning_message: Option<String>,
}

/**
 * Fetches the announce response from the tracker as buffer.
 * announce url example sintel.torrent: http://tracker.opentrackr.org:1337/scrape?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10
 */
pub async fn fetch_scrape_buffer(url: &str) -> Result<Vec<u8>> {
  let resp = reqwest::get(url).await?;

  if (resp.status().as_u16() / 100) != 2 {
    return Err(Error::msg(format!("Bad status code: {}", resp.status())));
  }

  let buffer =
    resp
      .bytes()
      .await?
      .to_vec();

  Ok(buffer)
}

pub fn fetch_scrape_buffer_to_struct(response: Vec<u8>) -> Result<TrackerScrapeResponse> {
  let response_struct = serde_bencode::from_bytes(&response)
    .with_context(|| {
      let str_resp = String::from_utf8_lossy(&response);
      format!("Bad tracker scrape response {}", str_resp)
    });
  
  Ok(response_struct?)
}

pub async fn fetch_scrape(url: &str) -> Result<TrackerScrapeNormalized> {
  let resp: Vec<u8> =
    fetch_scrape_buffer(url)
    .await
    .context("Failed to fetch scrape")?;
  let scrape_response =
    fetch_scrape_buffer_to_struct(resp)
      .context("Failed to parse scrape response")?;

  Ok(TrackerScrapeNormalized {
    url: url.to_string(),
    files: scrape_response.files,
    failure_reason: scrape_response.failure_reason,
    warning_message: scrape_response.warning_message
  })
}

pub async fn fetch_scrape_meta_info(meta_info: &crate::meta_info::MetaInfo) -> Result<Vec<TrackerScrapeNormalized>> {
  let info_hash_buffer = info_hash_buffer(&meta_info).context("Failed to get info hash")?;
  let url_encoded_info_hash = encode_binary(&info_hash_buffer);

  let tracker_urls = tracker_urls(meta_info);
  let scrape_urls = tracker_urls
    .iter()
    .map(|announce| {
      let mut url: String = announce.clone();
      url.push_str("scrape?info_hash=");
      url.push_str(&url_encoded_info_hash);
      url.replacen("udp", "http", 1)
    })
    .collect::<Vec<_>>();

  let responses =
    scrape_urls
      .iter()
      .map(|url| fetch_scrape(url))
      .collect::<Vec<_>>();

  let response = join_all(responses).await;
  let valid_responses = response
    .into_iter()
    .filter(|r| r.is_ok())
    .collect::<Vec<_>>()
    .into_iter()
    .map(|r| r.context("Failed to fetch scrape"))
    .collect::<Result<Vec<TrackerScrapeNormalized>>>()?;

  Ok(valid_responses)
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
      format!("Bad tracker announce response {}", str_resp)
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
              TrackerPeer {
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
