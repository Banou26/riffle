use anyhow::{Context, Result};

use serde_bencode::de;
use serde_bytes::ByteBuf;
use std::io::{Read};

#[derive(Debug, Deserialize)]
pub struct Node(String, i64);

#[derive(Debug, Deserialize)]
pub struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct MetaInfo {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
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
