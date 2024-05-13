use std::time::Instant;

use crossbeam_channel::Receiver;
use futures_signals::signal::{Mutable, MutableSignalCloned};

use crate::meta_info::{info_hash_hex, MetaInfo};


#[derive(Debug, Clone)]
pub struct Torrent {
  pub info_hash: String,
  pub meta_info: MetaInfo,
  pub inserted_at: Instant,
}

pub struct MakeTorrent {
  pub meta_info: MetaInfo,
  pub response_tx: crossbeam_channel::Sender<()>,
}

pub enum ClientMessage {
  RemoveTorrent(String)
}

pub fn make_torrent(torrent_options: MakeTorrent, rx: Receiver<ClientMessage>) -> MutableSignalCloned<Torrent>{
  let info_hash = info_hash_hex(&torrent_options.meta_info);
  let mut_torrent = Mutable::new(Torrent {
    info_hash: info_hash.clone(),
    meta_info: torrent_options.meta_info,
    inserted_at: std::time::Instant::now(),
});
  let mut lock = mut_torrent.lock_mut();

  rx.iter().for_each(|message| {
      match message {
          ClientMessage::RemoveTorrent(info_hash) => {
            
          }
      }
  });

  mut_torrent.signal_cloned()
}
