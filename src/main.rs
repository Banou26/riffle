#[macro_use]
extern crate serde_derive;

use anyhow::Result;

mod tui;
mod client;
mod tracker;
mod meta_info;
mod torrent;
mod utils;

use tui::{initialize_panic_handler, run, shutdown, startup};


#[tokio::main]
async fn main() -> Result<()> {
    // initialize_panic_handler();
    // startup()?;
    run().await?;
    // shutdown()?;
    Ok(())
}
