use std::{fs::File, io::Write};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::{Tree, TreeItem};

use crate::{
    app::{App, RequestCollection},
    data::write_dbg,
    ui::components::request_window,
};

use super::root_layout::create_root_layout;

pub fn render(frame: &mut Frame, state: &mut App) {
    let root_layout = create_root_layout(frame, "Convey");

    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Percentage(80)])
        .split(root_layout[1]);

    render_list(frame, &sections[0], state);
    request_window::render(frame, &sections[1], state);
}

fn render_list(frame: &mut Frame, area: &Rect, app_state: &mut App) {
    let items = construct_tree(&app_state.request_collections);
    let list = Tree::new(&items)
        .expect("all identifiers are unique")
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(match app_state.focused_section {
                    crate::app::FocusedSection::Left => Style::default(),
                    crate::app::FocusedSection::Right => Style::new().dark_gray(),
                })
                .title("Requests")
                .title_bottom(" (k) up (j) down (enter) select "),
        )
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>");

    if app_state.get_current_request().is_some() {
        println!("{:?}", app_state.get_current_request());
    }

    frame.render_stateful_widget(list, *area, &mut app_state.request_tree_state);
}

pub fn handle_key(key: KeyEvent, state: &mut App) {
    match state.focused_section {
        crate::app::FocusedSection::Left => match key.code {
            KeyCode::Char('j') => {
                state.request_tree_state.select_relative(|current| {
                    current.map_or(0, |current| current.saturating_add(1))
                });
            }
            KeyCode::Char('k') => {
                state.request_tree_state.select_relative(|current| {
                    current.map_or(0, |current| current.saturating_sub(1))
                });
            }
            KeyCode::Enter => {
                let contents = String::new();
                //contents = contents + state.request_tree_state.selected().to_owned();
                //write_dbg();
                let selected_id = if state.request_tree_state.selected().is_empty() {
                    return;
                } else {
                    state.request_tree_state.selected()[0].clone()
                };

                if let Some(request) = state.find_request(&selected_id) {
                    if let Some(collection) = state.find_collection_for_req_id(&request.id) {
                        println!("SADLAJS");
                        let request_id = request.id.clone();
                        let collection_id = collection.id.clone();

                        state.select_request(&request_id);
                        state.request_tree_state.select(vec![request_id]);
                        state.request_tree_state.open(vec![collection_id]);
                    }
                } else if state.find_collection(&selected_id).is_some() {
                    state.request_tree_state.toggle(vec![selected_id]);
                }
            }
            KeyCode::Char('l') => state.focused_section = crate::app::FocusedSection::Right,
            _ => {}
        },
        crate::app::FocusedSection::Right => match key.code {
            KeyCode::Char('h') => state.focused_section = crate::app::FocusedSection::Left,
            _ => request_window::handle_key(key, state),
        },
    }
}

fn construct_tree(collections: &Vec<RequestCollection>) -> Vec<TreeItem<'_, String>> {
    let mut items = Vec::new();

    for collection in collections {
        let mut children = Vec::new();
        for req in &collection.requests {
            children.push(TreeItem::new_leaf(req.id.clone(), req.url.clone()));
        }
        items.push(
            TreeItem::new(collection.id.clone(), collection.name.clone(), children).expect("error"),
        );
    }

    items
}
