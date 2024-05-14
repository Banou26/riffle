use std::time::Duration;

use anyhow::{Context, Result};
use crossbeam_channel::unbounded;
use futures_signals::{signal::SignalExt, signal_map::{MapDiff, MutableBTreeMap, SignalMapExt}};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc;

use crate::{client::{self, ClientMessage, TorrentClient}, meta_info::{read_meta_info_file, MetaInfo}, torrent::Torrent};

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
  counter: i64,
  should_quit: bool,
  ticker: i64,
  torrent_hashes: Vec<String>,
}

pub fn ui(f: &mut Frame, app: &mut App) {
  let area = f.size();
  f.render_widget(
    Paragraph::new(format!(
      "Press j or k to increment or decrement.\n\nCounter: {}\n\nTicker: {}\n\nTorrents: {:?}",
      app.counter, app.ticker, app.torrent_hashes
    ))
    .block(
      Block::default()
        .title("ratatui async counter app")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan))
    .alignment(Alignment::Center),
    area,
  );
}

#[derive(PartialEq)]
pub enum Action {
  ScheduleIncrement,
  ScheduleDecrement,
  Increment,
  Decrement,
  Quit,
  None,
}

pub fn update(app: &mut App, msg: Action) -> Action {
  match msg {
    Action::Increment => {
      app.counter += 1;
    },
    Action::Decrement => {
      app.counter -= 1;
    },
    Action::ScheduleIncrement => {
      let tx = app.action_tx.clone();
      tokio::spawn(async move {
        // tokio::time::sleep(Duration::from_secs(5)).await;
        tx.send(Action::Increment).unwrap();
      });
    },
    Action::ScheduleDecrement => {
      let tx = app.action_tx.clone();
      tokio::spawn(async move {
        // tokio::time::sleep(Duration::from_secs(5)).await;
        tx.send(Action::Decrement).unwrap();
      });
    },
    Action::Quit => app.should_quit = true, // You can handle cleanup and exit here
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
              crossterm::event::KeyCode::Char('j') => Action::ScheduleIncrement,
              crossterm::event::KeyCode::Char('k') => Action::ScheduleDecrement,
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
    // println!("CREATING TORRENT CLIENT");
    // let torrent_client = TorrentClient::new();
    // println!("LOADING META INFO");
    // let meta_info: MetaInfo =
    //   read_meta_info_file("./torrent_test.torrent")
    //     .context("Failed to read meta info file")?;


    // let cloned_torrent = torrent_client.clone();
    // tokio::spawn(async move {
    //     cloned_torrent.torrents_mut.clone().signal_map_cloned().for_each(|torrent| {
    //         println!("TORRENT SIGNAL{:?}", torrent);
    //         async {}
    //     });
    // });

    // let torrents_mut: MutableBTreeMap<String, Torrent> = MutableBTreeMap::new();
    // let lock = torrents_mut.lock_mut();


    // println!("ADDING TORRENT");
    // let torrent = torrent_client.add_torrent(meta_info).await?;
    // lock.insert(torrent.info_hash.clone(), torrent);

    
    let torrents_mut = MutableBTreeMap::new();

    let signal = torrents_mut.signal_map();
    tokio::spawn(async move {
        signal.for_each(|change| {
            println!("CHANGE {:?}", change);
            async {}
        }).await;
    });

    let mut lock = torrents_mut.lock_mut();
    lock.insert("foo", 1);


  // let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
  // let (action_tx, mut action_rx) = mpsc::unbounded_channel();
  // let mut app = App { counter: 0, should_quit: false, action_tx, ticker: 0, torrent_hashes: vec![] };
  // let task = handle_event(&app, app.action_tx.clone());
  // loop {
  //     app.torrent_hashes = torrent_client.torrents_mut.lock_mut().clone().into_keys().collect::<Vec<_>>();

  //     t.draw(|f| {
  //       ui(f, &mut app);
  //     })?;

  //     if let Some(action) = action_rx.recv().await {
  //       update(&mut app, action);
  //     }

  //     if app.should_quit {
  //       break;
  //     }
  //     app.ticker += 1;
  // }

  // task.abort();

  Ok(())
}
