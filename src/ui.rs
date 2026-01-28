use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::view::ViewType;

fn render_header(app: &mut App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = match &app.navigation.current {
        ViewType::ProfileSelect => "Welcome to Orbit TUI - Profile Select",
        ViewType::Projects => "Projects",
        ViewType::Targets { project } => &format!("{} > Targets", project),
        ViewType::Services { project, target } => &format!("{} > {} > Services", project, target),
        ViewType::Schema {
            project,
            target,
            service,
        } => &format!("{} > {} > {} > Schema", project, target, service),
    };
    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_content(app: &mut App, frame: &mut Frame, area: ratatui::layout::Rect) {
    if matches!(app.navigation.current, ViewType::Schema { .. }) {
        render_schema_viewer(app, frame, area);
        return;
    }
    let items: Vec<ListItem> = get_items_for_view(&app.navigation.current, app)
        .into_iter()
        .map(|s| ListItem::new(format!(" {}", s)))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_footer(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let help = if matches!(app.navigation.current, ViewType::Schema { .. }) {
        " j/k: scroll │ Tab: toggle supergraph │ Esc: back │ q: quit "
    } else if app.navigation.can_go_back() {
        " j/k: navigate │ Enter: select │ Esc: back │ q: quit "
    } else {
        " j/k: navigate │ Enter: select │ q: quit "
    };

    let footer = Paragraph::new(help).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}

fn render_schema_viewer(app: &mut App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = if app.showing_supergraph {
        "Supergraph Schema - Press TAB to Switch"
    } else {
        "Subgraph Schema - Press TAB to Switch"
    };
    let content = app
        .schema_content
        .as_deref()
        .unwrap_or("No schema available");
    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((app.scroll_offset, 0));
    frame.render_widget(paragraph, area);
}

pub fn get_items_for_view(view: &ViewType, app: &App) -> Vec<String> {
    match view {
        ViewType::ProfileSelect => {
            if app.profiles.is_empty() {
                vec!["(No profiles found)".into()]
            } else {
                app.profiles.clone()
            }
        }
        ViewType::Projects => {
            if app.projects.is_empty() {
                vec!["(No projects found)".into()]
            } else {
                app.projects.clone()
            }
        }
        ViewType::Targets { .. } => {
            if app.targets.is_empty() {
                vec!["(No targets found)".into()]
            } else {
                app.targets.clone()
            }
        }
        ViewType::Services { .. } => {
            if app.services.is_empty() {
                vec!["(No services found)".into()]
            } else {
                app.services.clone()
            }
        }
        ViewType::Schema { .. } => {
            vec!["View SDL".into(), "View Supergraph".into()]
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_content(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);
}
