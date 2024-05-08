mod meta_info;
mod tracker;
mod utils;

use meta_info::{read_meta_info_file, render_meta_info, info_hash_hex, info_hash_buffer};
use tracker::{announce_buffer};
use utils::{read_file};

use urlencoding::{encode_binary};

#[macro_use]
extern crate serde_derive;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meta_info: meta_info::MetaInfo = read_meta_info_file("./sintel.torrent").unwrap();
    let info_hash = info_hash_hex(&meta_info);
    let info_hash_buffer = info_hash_buffer(&meta_info).unwrap();
    render_meta_info(&meta_info);
    println!("infohash: {:?}", info_hash);

    let result = encode_binary(&info_hash_buffer);
    println!("infohash uri: {}", result);

    let announce_url = "http://tracker.opentrackr.org:1337/announce?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10";
    let resp = announce_buffer(announce_url).await?;
    // let resp = read_file("./announce").unwrap();
    
    let tracker_response = tracker::announce_response_to_struct(resp).unwrap();
    println!("{:?}", tracker_response);

    Ok(())
}
