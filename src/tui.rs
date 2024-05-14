use std::time::Duration;

use anyhow::{Context, Result};
use crossbeam_channel::unbounded;
use futures_signals::{signal::SignalExt, signal_map::{MapDiff, MutableBTreeMap, SignalMapExt}};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc;

use crate::{client::{TorrentClient}, meta_info::{read_meta_info_file, MetaInfo}, torrent::Torrent};

pub fn initialize_panic_handler() {
  let original_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    shutdown().unwrap();
    original_hook(panic_info);
  }));
}

pub fn startup() -> Result<()> {
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  Ok(())
}

pub fn shutdown() -> Result<()> {
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

pub struct App {
  action_tx: mpsc::UnboundedSender<Action>,
  should_quit: bool,
  torrent_client: TorrentClient,
}

pub fn ui(f: &mut Frame, app: &mut App) {
  let torrent_hashes = app.torrent_client.torrents.clone().into_keys().collect::<Vec<_>>();

  let area = f.size();
  f.render_widget(
    Paragraph::new(format!(
      "\n\nTorrents: {:#?}",
      torrent_hashes
    ))
    .block(
      Block::default()
        .title("Riffle")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan)),
    area,
  );
}

#[derive(PartialEq)]
pub enum Action {
  Quit,
  None,
}

pub fn update(app: &mut App, msg: Action) -> Action {
  match msg {
    Action::Quit => app.should_quit = true,
    _ => {},
  };
  Action::None
}

pub fn handle_event(app: &App, tx: mpsc::UnboundedSender<Action>) -> tokio::task::JoinHandle<()> {
  let tick_rate = std::time::Duration::from_millis(250);
  tokio::spawn(async move {
    loop {
      let action = if crossterm::event::poll(tick_rate).unwrap() {
        if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
          if key.kind == crossterm::event::KeyEventKind::Press {
            match key.code {
              crossterm::event::KeyCode::Char('q') => Action::Quit,
              _ => Action::None,
            }
          } else {
            Action::None
          }
        } else {
          Action::None
        }
      } else {
        Action::None
      };
      if let Err(_) = tx.send(action) {
        break;
      }
    }
  })
}

pub async fn run() -> Result<()> {
    let mut torrent_client = TorrentClient::new();

    let meta_info: MetaInfo =
      read_meta_info_file("./torrent_test.torrent")
        .context("Failed to read meta info file")?;
    torrent_client.add_torrent(meta_info);

    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let mut app = App { should_quit: false, action_tx, torrent_client };
    let task = handle_event(&app, app.action_tx.clone());
    loop {
        t.draw(|f| {
          ui(f, &mut app);
        })?;

        if let Some(action) = action_rx.recv().await {
          update(&mut app, action);
        }

        if app.should_quit {
          break;
        }
    }

    task.abort();

    Ok(())
}
