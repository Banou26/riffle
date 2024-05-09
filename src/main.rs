#[macro_use]
extern crate serde_derive;

use futures::{FutureExt, StreamExt};
use urlencoding::{encode_binary};
use anyhow::Result;

mod meta_info;
mod tracker;
mod utils;
mod peer;
mod bitfield;

use meta_info::{read_meta_info_file, render_meta_info, info_hash_hex, info_hash_buffer};
use tracker::{fetch_announce_buffer_to_struct, stream_scrape_meta_info, fetch_scrape_meta_info, fetch_announce};
use utils::{read_file};

#[tokio::main]
async fn main() -> Result<()> {
    let meta_info: meta_info::MetaInfo = read_meta_info_file("./torrent_test.torrent").unwrap();
    let info_hash = info_hash_hex(&meta_info);
    let info_hash_buffer = info_hash_buffer(&meta_info).unwrap();
    render_meta_info(&meta_info);
    println!("infohash: {:?}", info_hash);

    let result = encode_binary(&info_hash_buffer);
    println!("infohash uri: {}", result);


    let scrapes_stream = stream_scrape_meta_info(&meta_info)?;

    // loop through the scrapes stream and log the results
    scrapes_stream.for_each(|scrape| async move {
        println!("scrape: {} {}", scrape.url, scrape.response.is_ok());
    }).await;




    // let scrape_responses = fetch_scrape_meta_info(&meta_info).await?;
    // // println!("scrapes: {:?}", scrape_responses);

    // let first_scrape = scrape_responses.first().unwrap();




    // let announce_response = fetch_announce(&first_scrape.announce_url).await?;
    // println!("first scrape announce: {:?}", announce_response);

    // let scrape_url = "http://tracker.opentrackr.org:1337/scrape?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10";
    // let scrape_response = tracker::fetch_scrape(scrape_url).await?;

    // let resp = read_file("./scrape").unwrap();
    // let scrape_response = tracker::fetch_scrape_buffer_to_struct(resp).unwrap();
    // println!("{:?}", scrape_response);


    // let annnounce_url = "http://tracker.opentrackr.org:1337/announce?info_hash=%08%AD%A5%A7%A6%18%3A%AE%1E%09%D81%DFgH%D5f%09Z%10";
    // let announce_response = fetch_announce(annnounce_url).await?;

    // let resp = read_file("./announce").unwrap();
    // let announce_response = announce::fetch_announce_buffer_to_struct(resp).unwrap();
    // println!("{:?}", announce_response);

    Ok(())
}
