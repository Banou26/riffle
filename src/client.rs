use std::clone;
use std::collections::BTreeMap;

use anyhow::{Context, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_signals::signal::{Mutable};
use futures_signals::signal_map::{MapDiff, MutableBTreeMap};

use crate::meta_info::{info_hash_hex, MetaInfo};
use crate::torrent::Torrent;

pub struct AddTorrentOptions {
    pub meta_info: MetaInfo,
    pub response_tx: crossbeam_channel::Sender<()>,
}

pub enum ClientMessage {
    AddTorrent(AddTorrentOptions),
    RemoveTorrent(String)
}

#[derive(Debug, Clone)]
pub struct TorrentClient {
    tx: Sender<ClientMessage>,
    rx: Receiver<ClientMessage>,
    pub torrents: BTreeMap<String, Torrent>,
}

impl TorrentClient {
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<ClientMessage>();
        let torrents = BTreeMap::new();

        Self {
            tx,
            rx,
            torrents
        }
    }

    // pub async fn add_torrent(&self, meta_info: MetaInfo) -> Result<Torrent>{
    //     let info_hash = info_hash_hex(&meta_info);
    //     let torrent = Torrent {
    //         info_hash: info_hash.clone(),
    //         meta_info,
    //         inserted_at: std::time::Instant::now(),
    //     };
    //     &self.torrents.push(torrent.clone());
    //     Ok(torrent)
    // }

    // pub fn remove_torrent(&self, info_hash: String) {
    //     self.torrents.lock_mut().remove(&info_hash);
    // }
}

pub fn add_torrent(torrent_client: &mut TorrentClient, meta_info: MetaInfo) -> Result<Torrent>{
    let info_hash = info_hash_hex(&meta_info);
    let torrent = Torrent {
        info_hash: info_hash.clone(),
        meta_info,
        inserted_at: std::time::Instant::now(),
    };
    torrent_client.torrents.insert(torrent.info_hash.clone(), torrent.clone());
    Ok(torrent)
}

// pub fn make_client(rx: Receiver<ClientMessage>) -> Mutable<TorrentClient> {
//     let torrents: MutableBTreeMap<String, Torrent> = MutableBTreeMap::new();
//     let client = Mutable::new(TorrentClient {
//         torrents: torrents.clone(),
//     });

//     tokio::spawn(async move {
//         rx.iter().for_each(|message| {
//             let lock = torrents.lock_mut();
//             match message {
//                 ClientMessage::AddTorrent(add_torrent) => {
//                     let info_hash = info_hash_hex(&add_torrent.meta_info);
//                     let torrent = Torrent {
//                         info_hash: info_hash.clone(),
//                         meta_info: add_torrent.meta_info,
//                         inserted_at: std::time::Instant::now(),
//                     };
//                     lock.clone().insert(info_hash, torrent);
//                     add_torrent.response_tx.send(()).unwrap();
//                 },
//                 ClientMessage::RemoveTorrent(info_hash) => {
//                     lock.clone().remove(&info_hash);
//                 }
//             }
//         })
//     });

//     client
// }

// pub fn add_torrent(client: &Mutable<TorrentClient>, add_torrent: AddTorrent) {
//     let message = ClientMessage::AddTorrent(add_torrent);
//     client.lock_mut().torrents.lock_mut().signal(message);
// }