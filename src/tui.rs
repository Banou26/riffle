use std::cmp;

use anyhow::Result;
use humansize::{format_size, DECIMAL};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc;

use crate::{client::TorrentClient, meta_info::MetaInfo};

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

pub fn render_root_box(f: &mut Frame, area: Rect) -> Rect {
    let outer = Block::default()
        .borders(Borders::ALL)
        .title("Riffle")
        .title_alignment(Alignment::Center);

    let inner = outer.inner(area);
    f.render_widget(outer, area);

    inner
}

pub fn render_torrent_selection(f: &mut Frame, app: &App, area: Rect) {
    let mut state = ListState::default().with_selected(Some(0));

    let header_widget = List::new(
        app
            .torrent_client
            .torrents
            .values()
            .map(|torrent| {
                let current_torrent_file_sizes = torrent
                    .meta_info
                    .info
                    .files
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|file| file.length)
                    .reduce(|x, y| x + y)
                    .unwrap_or(0);
              
                let current_torrent_total_size = torrent
                    .meta_info
                    .info
                    .length
                    .unwrap_or(current_torrent_file_sizes);
              
                let current_torrent_total_size_u64 = u64::try_from(current_torrent_total_size).unwrap();
              
                let current_torrent_size = format_size(
                  current_torrent_total_size_u64,
                    DECIMAL,
                );

                ListItem::new(Span::raw(format!(
                    "{} ({})",
                    torrent.meta_info.info.name,
                    current_torrent_size
                )))
              })
              .collect::<Vec<_>>()
    )
    .highlight_symbol(">>")
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::BOTTOM),
    )
    .style(Style::default().fg(Color::Cyan));

    f.render_stateful_widget(header_widget, area, &mut state);
}

pub fn render_pieces(f: &mut Frame, app: &App, area: Rect) {
    let torrent_hashes = app
        .torrent_client
        .torrents
        .clone()
        .into_keys()
        .collect::<Vec<_>>();

    let selected_torrent = app
        .torrent_client
        .torrents
        .get(torrent_hashes[0].as_str())
        .unwrap();

    let chunks_lines = u16::try_from(selected_torrent.info().pieces.len() / area.width as usize).unwrap();

    let pieces =
        Paragraph::new( vec![0; chunks_lines.into()].iter().map(|x| "█").collect::<String>())
            .wrap(Wrap { trim: false });

    f.render_widget(pieces, area);
}

pub fn render_torrent_info(f: &mut Frame, app: &App, area: Rect) {
    let torrent_hashes = app
        .torrent_client
        .torrents
        .clone()
        .into_keys()
        .collect::<Vec<_>>();

    let selected_torrent = app
        .torrent_client
        .torrents
        .get(torrent_hashes[0].as_str())
        .unwrap();

    let current_torrent_file_sizes = selected_torrent
        .meta_info
        .info
        .files
        .clone()
        .unwrap()
        .iter()
        .map(|file| file.length)
        .reduce(|x, y| x + y)
        .unwrap_or(0);

    let current_torrent_total_size = selected_torrent
        .meta_info
        .info
        .length
        .unwrap_or(current_torrent_file_sizes);

    let current_torrent_total_size_u64 = u64::try_from(current_torrent_total_size).unwrap();

    let current_torrent_size = format_size(
      current_torrent_total_size_u64,
        DECIMAL,
    );

    let selected_torrent_widget = Paragraph::new(selected_torrent.meta_info.info.name.clone())
        .alignment(Alignment::Center);

    let chunks_lines = u16::try_from(selected_torrent.info().pieces.len() / area.width as usize).unwrap();

    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        //   Constraint::Percentage(90),
          Constraint::Min(2),
          Constraint::Length(chunks_lines),
      ])
      .split(area);

    f.render_widget(selected_torrent_widget, chunks[0]);
    render_pieces(f, app, chunks[1]);

}

pub fn draw(f: &mut Frame, app: &App) {
    let size = render_root_box(f, f.size());
    
    let torrent_hashes = app
        .torrent_client
        .torrents
        .clone()
        .into_keys()
        .collect::<Vec<_>>();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(3),
            Constraint::Percentage(100),
        ])
        .split(size);

    render_torrent_selection(f, app, chunks[0]);
    render_torrent_info(f, app, chunks[1]);
}

pub fn ui(f: &mut Frame, app: &mut App) {
    draw(f, app);
}

#[derive(PartialEq)]
pub enum Action {
    Quit,
    None,
}

pub fn update(app: &mut App, msg: Action) -> Action {
    match msg {
        Action::Quit => app.should_quit = true,
        _ => {}
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

    let meta_info = MetaInfo::from_file("./torrent_test.torrent")?;
    torrent_client.add_torrent(meta_info);

    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let mut app = App {
        should_quit: false,
        action_tx,
        torrent_client,
    };
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
