use std::time::Instant;

use crate::meta_info::MetaInfo;

pub struct Torrent {
  pub info_hash: String,
  pub meta_info: MetaInfo,
  pub inserted_at: Instant,
}
