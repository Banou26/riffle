#[macro_use]
extern crate serde_derive;

use futures::{FutureExt, StreamExt};
use reqwest::Url;
use urlencoding::{encode_binary};
use anyhow::Result;

mod meta_info;
mod tracker;
mod utils;
mod peer;
mod bitfield;
mod client;
mod torrent;

use meta_info::{read_meta_info_file, render_meta_info, info_hash_hex, info_hash_buffer};
use tracker::{fetch_announce_buffer_to_struct, stream_scrape_meta_info, fetch_scrape_meta_info, fetch_announce};
use utils::{read_file};
use crossbeam_channel::{unbounded, Sender};
// use crate::client::Client;
// use crate::torrent::Torrent;

use futures_signals::signal_map::{SignalMapExt, MapDiff, MutableBTreeMap};

#[derive(Clone, Debug)]
struct Callback {
    pub str: String
}

#[derive(Clone, Debug)]
struct TestStruct {
    pub str: String,
    pub callback: Sender<Callback>
}

#[tokio::main]
async fn main() -> Result<()> {
    let map: MutableBTreeMap<&str, i32> = MutableBTreeMap::new();
    let signal = map.signal_map();

    tokio::spawn(async move {
        let _ = signal.for_each(|change| {
            match change {
                MapDiff::Replace { entries } => {
                    println!("Replace: {:?}", entries);
                },
                MapDiff::Insert { key, value } => {
                    println!("Insert: {:?} {:?}", key, value);
                },
                MapDiff::Update { key, value } => {
                    println!("Update: {:?} {:?}", key, value);
                },
                MapDiff::Remove { key } => {
                    println!("Remove: {:?}", key);
                },
                MapDiff::Clear {} => {
                    println!("Clear");
                },
            }
            async {}
        }).await;
    });


    let mut lock = map.lock_mut();
    lock.insert("foo", 5);
    lock.insert("bar", 10);



    let (s, r) = unbounded();


    let (s_callback, r_callback) = unbounded();
    

    s.send(TestStruct { callback: s_callback, str: "foo".to_string() }).unwrap();


    let call = r.recv().unwrap();
    call.callback.send(Callback { str: "bar".to_string() }).unwrap();

    let call_response = r_callback.recv().unwrap();
    // Receive the message from the channel.
    println!("call {:?}", call.str);
    println!("call response {:?}", call_response.str);

    // let client = Client::new();

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
        let url = Url::parse(&scrape.url).unwrap();
        let hostname = url.host_str().unwrap();
        let is_ok = match scrape.response.is_ok() {
            true => "ok",
            false => "failed"
        };

        if scrape.response.is_err() {
            println!("{} {}", hostname, scrape.response.unwrap_err());
            return
        }

        let files = scrape
            .response
            .unwrap()
            .files
            .iter()
            .map(|file_tuple| file_tuple.1.clone())
            .collect::<Vec<tracker::TrackerScrapeResponseFile>>();

        let mut files_incomplete =
            files
                .iter()
                .map(|file| file.incomplete)
                .collect::<Vec<i64>>();
        files_incomplete.sort();

        let mut files_complete =
            files
                .iter()
                .map(|file| file.complete)
                .collect::<Vec<i64>>();
        files_complete.sort();

        let leechers =
            files_incomplete.last().unwrap_or(&0).to_owned();

        let seeders =
            files_complete.last().unwrap_or(&0).to_owned();

        println!("{} {} seeders: {} leechers: {}", hostname, is_ok, seeders, leechers);
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
