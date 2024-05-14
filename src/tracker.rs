use std::{collections::BTreeMap, net::Ipv4Addr, time::Instant};

use futures::stream::FuturesUnordered;
use futures::Future;
use futures::future::join_all;
use anyhow::{Context, Result};
use serde_bytes::ByteBuf;
use urlencoding::encode_binary;

use crate::meta_info::MetaInfo;
use crate::utils::fetch_buffer;
use crate::utils::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackerPeer {
    #[serde(default)]
    #[serde(rename = "peer id")]
    pub peer_id: Option<String>,
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Peers {
    ByteBuf(ByteBuf),
    PeerStruct(Vec<TrackerPeer>)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackerScrapeResponseFile {
    pub complete: i64,
    #[serde(default)]
    pub downloaded: Option<i64>,
    pub incomplete: i64,
    #[serde(default)]
    pub name: Option<String>
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackerScrapeResponse {
    pub files: BTreeMap<ByteBuf, TrackerScrapeResponseFile>,

    #[serde(default)]
    #[serde(rename = "failure reason")]
    pub failure_reason: Option<String>,

    #[serde(default)]
    #[serde(rename = "warning message")]
    pub warning_message: Option<String>,
}

#[derive(Debug)]
pub struct Scrape {
    pub url: String,
    pub at: Instant,
    pub response: Result<TrackerScrapeResponse>,
}

impl Scrape {
    pub fn is_ok(&self) -> bool {
        self.response.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.response.is_err()
    }

    fn parse_buffer(response: Vec<u8>) -> Result<TrackerScrapeResponse> {
        serde_bencode::from_bytes(&response)
            .context(format!("Bad tracker scrape response {}", String::from_utf8_lossy(&response)))
    }

    pub async fn from_url(url: String) -> Self {
        let response =
            fetch_buffer(url.as_str())
                .await
                .context(format!("Failed to fetch scrape from {}", url))
                .and_then(Scrape::parse_buffer)
                .context(format!("Failed to parse scrape response from {}", url));

        Scrape {
            url: url.to_string(),
            at: Instant::now(),
            response
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tracker {
    pub url: String
}

impl Tracker {
    pub fn new(url: String) -> Self {
        Self {
            url
        }
    }

    pub async fn fetch_announce(&mut self) -> Result<()> {
        let response = fetch_announce(&self.url).await?;
        Ok(())
    }

    pub async fn fetch_scrape(&mut self) -> Result<Scrape> {
        let response = Scrape::from_url(self.url.clone()).await;
        Ok(response)
    }
}

pub async fn fetch_scrape_meta_info(meta_info: &crate::meta_info::MetaInfo) -> Result<Vec<Scrape>> {
    let scrape_urls = meta_info.scrape_urls()?;

    let responses =
        scrape_urls
            .iter()
            .map(|url| Scrape::from_url(url.clone()))
            .collect::<Vec<_>>();

    let response = join_all(responses).await;
    Ok(response)
}

pub fn stream_scrape_meta_info(meta_info: &crate::meta_info::MetaInfo) -> Result<FuturesUnordered<impl Future<Output = Scrape>>> {
    let scrape_urls = meta_info.scrape_urls()?;

    let futures: FuturesUnordered<_> =
      scrape_urls
        .iter()
        .map(|url| Scrape::from_url(url.clone()))
        .collect();

    Ok(futures)
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
