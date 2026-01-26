mod action;
mod app;
mod input;
mod tui;
mod ui;
mod view;

use crossterm::event::{Event, KeyEventKind};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    tui::install_panic_hook();

    let mut terminal = tui::init()?;
    let mut app = app::App::new();

    while app.running {
        terminal.draw(|frame| ui::render(&mut app, frame))?;
        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(action) = input::handle_key_event(key) {
                    app.update(action).await;
                }
            }
        }
    }
    tui::restore()?;
    Ok(())
}
