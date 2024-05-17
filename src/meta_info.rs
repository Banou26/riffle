use anyhow::{Context, Result};
use sha1::{Digest, Sha1};

use serde_bencode::de;
use serde_bencode::ser;
use serde_bytes::ByteBuf;
use std::borrow::Cow;
use std::io::Read;
use urlencoding::encode_binary;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node(String, i64);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

impl Info {
    pub fn to_buffer(&self) -> Result<Vec<u8>> {
        ser::to_bytes(self).context("Failed to serialize info")
    }

    pub fn to_hash_buffer(&self) -> Result<Vec<u8>> {
        let buffer = self.to_buffer()?;
        let mut hasher = Sha1::new();
        hasher.update(buffer);
        Ok(hasher.finalize().as_slice().to_vec())
    }

    pub fn to_hash_hex(&self) -> String {
        let buffer = self.to_hash_buffer().unwrap();
        hex::encode(buffer)
    }

    pub fn to_hash(&self) -> String {
        self.to_hash_hex()
    }

    pub fn to_url_encoded(&self) -> Result<String> {
        let info_hash_buffer = self.to_buffer().context("Failed to get info hash")?;
        Ok(encode_binary(&info_hash_buffer).to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaInfo {
    pub info: Info,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

impl MetaInfo {
    pub fn to_info_hash(&self) -> String {
        self.info.to_hash()
    }

    pub fn tracker_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();
        if let Some(announce) = &self.announce {
            urls.push(announce.replace("announce", ""));
        }
        if let Some(announce_list) = &self.announce_list {
            for announce in announce_list {
                urls.push(announce[0].replace("announce", ""));
            }
        }
        urls
    }

    pub fn from_buffer(buffer: &Vec<u8>) -> Result<MetaInfo> {
        de::from_bytes::<MetaInfo>(&buffer).context("Failed to parse meta info buffer")
    }

    pub fn from_file(str: &str) -> Result<MetaInfo> {
        let mut file = std::fs::File::open(str)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        MetaInfo::from_buffer(&buffer)
    }

    pub fn scrape_urls(&self) -> Result<Vec<String>> {
        let url_encoded_info_hash = self.info.to_url_encoded()?;
        let scrape_urls = MetaInfo::tracker_urls(self)
            .iter()
            .map(|announce| {
                let mut url: String = announce.clone();
                url.push_str("scrape?info_hash=");
                url.push_str(&url_encoded_info_hash);
                url.replacen("udp", "http", 1)
            })
            .collect::<Vec<_>>();
        Ok(scrape_urls)
    }
}
