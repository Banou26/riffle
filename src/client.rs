use futures_signals::signal_map::{MutableBTreeMap, MutableBTreeMapLockMut};
use futures_signals::signal_map::{SignalMapExt, MapDiff};

use crate::torrent::Torrent;


pub struct Client {
  pub torrents: MutableBTreeMap<String, Torrent>,
}

impl Client {
  pub fn lock_torrents(&self) -> MutableBTreeMapLockMut<'_, String, Torrent> {
    self.torrents.lock_mut()
  }
}

impl Default for Client {
  fn default() -> Self {
      Self {
        torrents: MutableBTreeMap::new()
      }
  }
}


// impl Default for Client {
//   fn default() -> Self {
//     let torrents = MutableBTreeMap::new();
//     let torrents_lock = torrents.lock_mut();
//     Self {
//       torrents: torrents,
//       torrents_lock
//     }
//   }
// }