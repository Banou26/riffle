use std::collections::BTreeMap;

use anyhow::Result;

use crate::meta_info::{info_hash_hex, MetaInfo};
use crate::torrent::Torrent;

#[derive(Debug, Clone)]
pub struct TorrentClient {
    pub torrents: BTreeMap<String, Torrent>,
}

impl TorrentClient {
    pub fn new() -> Self {
        Self {
            torrents: BTreeMap::new()
        }
    }

    pub fn add_torrent(self: &mut TorrentClient, meta_info: MetaInfo) -> Result<Torrent>{
        let info_hash = info_hash_hex(&meta_info);
        let torrent = Torrent {
            info_hash: info_hash.clone(),
            meta_info,
            inserted_at: std::time::Instant::now(),
        };
        self.torrents.insert(torrent.info_hash.clone(), torrent.clone());
        Ok(torrent)
    }
}
