#[macro_use]
extern crate serde_derive;

use anyhow::Result;

mod bitfield;
mod client;
mod meta_info;
mod peer;
mod torrent;
mod tracker;
mod tui;
mod utils;
mod piece_picker;

use tui::{initialize_panic_handler, run, shutdown, startup};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler();
    startup()?;
    run().await?;
    shutdown()?;
    Ok(())
}
