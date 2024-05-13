use anyhow::{Context, Result};
use sha1::{Sha1, Digest};

use serde_bencode::de;
use serde_bencode::ser;
use serde_bytes::ByteBuf;
use std::io::{Read};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node(String, i64);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
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

pub fn render_meta_info(meta_info: &MetaInfo) {
    println!("name:\t\t{}", meta_info.info.name);
    println!("announce:\t{:?}", meta_info.announce);
    println!("nodes:\t\t{:?}", meta_info.nodes);
    if let Some(al) = &meta_info.announce_list {
        for a in al {
            println!("announce list:\t{}", a[0]);
        }
    }
    println!("httpseeds:\t{:?}", meta_info.httpseeds);
    println!("creation date:\t{:?}", meta_info.creation_date);
    println!("comment:\t{:?}", meta_info.comment);
    println!("created by:\t{:?}", meta_info.created_by);
    println!("encoding:\t{:?}", meta_info.encoding);
    println!("piece length:\t{:?}", meta_info.info.piece_length);
    println!("private:\t{:?}", meta_info.info.private);
    println!("root hash:\t{:?}", meta_info.info.root_hash);
    println!("md5sum:\t\t{:?}", meta_info.info.md5sum);
    println!("path:\t\t{:?}", meta_info.info.path);
    if let Some(files) = &meta_info.info.files {
        for f in files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
            println!("file md5sum:\t{:?}", f.md5sum);
        }
    }
}

pub fn parse_meta_info_buffer(buffer: &Vec<u8>) -> Result<MetaInfo> {
    de::from_bytes::<MetaInfo>(&buffer)
        .context("Failed to parse meta info buffer")
}

pub fn read_meta_info_file(str: &str) -> Result<MetaInfo> {
    let mut file = std::fs::File::open(str)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    parse_meta_info_buffer(&buffer)
}

pub fn info_buffer(info: &Info) -> Result<Vec<u8>> {
    ser::to_bytes(info)
        .context("Failed to serialize info")
}

pub fn info_buffer_hash(buffer: &Vec<u8>) -> Result<Vec<u8>> {
    let mut hasher = Sha1::new();
    hasher.update(buffer);
    Ok(
        hasher
            .finalize()
            .as_slice()
            .to_vec()
    )
}

pub fn info_hash_buffer(meta_info: &MetaInfo) -> Result<Vec<u8>> {
    let buffer = info_buffer(&meta_info.info).unwrap();
    info_buffer_hash(&buffer)
}

pub fn info_hash_hex(meta_info: &MetaInfo) -> String {
    let buffer = info_hash_buffer(meta_info).unwrap();
    hex::encode(buffer)
}

pub fn tracker_urls(meta_info: &MetaInfo) -> Vec<String> {
    let mut urls = Vec::new();
    if let Some(announce) = &meta_info.announce {
        urls.push(announce.replace("announce", ""));
    }
    if let Some(announce_list) = &meta_info.announce_list {
        for announce in announce_list {
            urls.push(announce[0].replace("announce", ""));
        }
    }
    urls
}
