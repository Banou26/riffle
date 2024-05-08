mod meta_info;
use meta_info::{read_meta_info_file, render_meta_info};

#[macro_use]
extern crate serde_derive;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_meta_info_file("./sintel.torrent").unwrap();
    render_meta_info(&file);

    Ok(())
}
