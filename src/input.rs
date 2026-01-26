use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::action::Action;

pub fn handle_key_event(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('j') | KeyCode::Up => Some(Action::NavigateUp),
        KeyCode::Char('k') | KeyCode::Down => Some(Action::NavigateDown),
        KeyCode::Enter => Some(Action::Select),
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Action::Quit),
        _ => None,
    }
}