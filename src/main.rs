#[macro_use]
extern crate serde_derive;

use std::{fs::File, io::Read};
use serde_bencode::de;

mod parse_torrent;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./sintel.torrent").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let file = de::from_bytes::<parse_torrent::Torrent>(&contents).unwrap();
    parse_torrent::render_torrent(&file);

    Ok(())
}
