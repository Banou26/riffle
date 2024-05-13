use crossbeam_channel::Receiver;
use futures_signals::signal_map::{MutableBTreeMap, MutableSignalMap};

use crate::meta_info::{info_hash_hex, MetaInfo};
use crate::torrent::Torrent;

pub struct AddTorrent {
    pub meta_info: MetaInfo,
    pub response_tx: crossbeam_channel::Sender<()>,
}

pub enum ClientMessage {
    AddTorrent(AddTorrent),
    RemoveTorrent(String)
}

pub fn make_client(rx: Receiver<ClientMessage>) -> MutableSignalMap<String, Torrent>{
    let torrents: MutableBTreeMap<String, Torrent> = MutableBTreeMap::new();
    let mut lock = torrents.lock_mut();

    rx.iter().for_each(|message| {
        match message {
            ClientMessage::AddTorrent(add_torrent) => {
                let info_hash = info_hash_hex(&add_torrent.meta_info);
                let torrent = Torrent {
                    info_hash: info_hash.clone(),
                    meta_info: add_torrent.meta_info,
                    inserted_at: std::time::Instant::now(),
                };
                lock.clone().insert(info_hash, torrent);
                add_torrent.response_tx.send(()).unwrap();
            },
            ClientMessage::RemoveTorrent(info_hash) => {
                lock.clone().remove(&info_hash);
            }
        }
    });

    torrents.signal_map_cloned()
}
