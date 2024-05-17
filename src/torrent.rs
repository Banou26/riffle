use std::time::Instant;

use crate::bitfield::BitField;
use crate::meta_info::{Info, MetaInfo};
use crate::peer::PeerWire;

#[derive(Debug, Clone)]
pub struct Torrent {
    pub info_hash: String,
    pub meta_info: MetaInfo,
    pub inserted_at: Option<Instant>,
    pub pieces_bitfield: BitField,
    pub peers: Vec<PeerWire>,
}

impl Torrent {
    pub fn new(meta_info: MetaInfo) -> Self {
        let info_hash = meta_info.to_info_hash();
        let pieces_bitfield = BitField::new(meta_info.info.pieces.len() / 20);
        Self {
            info_hash,
            meta_info,
            inserted_at: Some(Instant::now()),
            pieces_bitfield,
            peers: vec![],
        }
    }

    pub fn info_hash(&self) -> String {
        self.meta_info.to_info_hash()
    }

    pub fn meta_info(&self) -> MetaInfo {
        self.meta_info.clone()
    }

    pub fn info(&self) -> Info {
        self.meta_info.info.clone()
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

    pub fn downloaded_pieces(&self) -> u64 {
        self.pieces_bitfield.iter().fold(0, |x, y| if y { x + 1 } else { x })
    }
}
