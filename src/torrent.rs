use std::time::Instant;

use crate::meta_info::MetaInfo;
use crate::peer::PeerWire;

#[derive(Debug, Clone)]
pub struct Torrent {
  pub info_hash: String,
  pub meta_info: MetaInfo,
  pub inserted_at: Option<Instant>,
  pub peers: Vec<PeerWire>
}

impl Torrent {
  pub fn new(meta_info: MetaInfo) -> Self {
    let info_hash = meta_info.to_info_hash();
    Self {
      info_hash,
      meta_info,
      inserted_at: Some(Instant::now()),
      peers: vec![]
    }
  }

  pub fn info_hash(&self) -> String {
    self.meta_info.to_info_hash()
  }

  pub fn meta_info(&self) -> MetaInfo {
    self.meta_info.clone()
  }

  pub fn set_meta_info(&mut self, meta_info: MetaInfo) {
    self.meta_info = meta_info;
  }

  pub fn inserted_at(&self) -> Option<Instant> {
    self.inserted_at.clone()
  }

  pub fn set_inserted_at(&mut self, inserted_at: Instant) {
    self.inserted_at = Some(inserted_at);
  }
}
