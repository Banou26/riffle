use std::collections::BTreeMap;

use crate::meta_info::MetaInfo;
use crate::torrent::Torrent;

#[derive(Debug, Clone)]
pub struct TorrentClient {
    pub torrents: BTreeMap<String, Torrent>,
}

impl TorrentClient {
    pub fn new() -> Self {
        Self {
            torrents: BTreeMap::new(),
        }
    }

    pub fn add_torrent(self: &mut TorrentClient, meta_info: MetaInfo) {
        let torrent = Torrent::new(meta_info);
        self.torrents.insert(torrent.info_hash(), torrent.clone());
    }
}
