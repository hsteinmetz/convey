use std::collections::HashMap;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, ListState, Paragraph},
    Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::app::{App, RequestData};

use super::root_layout::create_root_layout;

pub fn render(frame: &mut Frame, state: &mut App) {
    let root_layout = create_root_layout(frame, "Convey");

    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Percentage(80)])
        .split(root_layout[1]);

    let right = Block::bordered().style(Style::default());

    render_list(frame, &sections[0], state);
    frame.render_widget(right, sections[1]);
}

fn render_list(frame: &mut Frame, area: &Rect, app_state: &mut App) {
    let mut collections = HashMap::new();
    collections.insert(
        String::from("Collection 1"),
        vec![RequestData::new(), RequestData::new()],
    );
    collections.insert(
        String::from("Collection 2"),
        vec![RequestData::new(), RequestData::new()],
    );
    app_state.request_collections = collections;
    let items = construct_tree(&app_state.request_collections);
    let list = Tree::new(&items)
        .expect("all identifiers are unique")
        .block(Block::bordered().title("Requests"))
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>");

    frame.render_stateful_widget(list, *area, &mut app_state.request_tree_state);
}

pub fn handle_key(key: KeyEvent, state: &mut App) {
    match &state.focused_section {
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
                let selected_identifier = state.request_tree_state.selected();

                state
                    .request_tree_state
                    .toggle(selected_identifier.to_vec());
            }
            _ => {}
        },
        crate::app::FocusedSection::Right => {}
    }
}

fn construct_tree(map: &HashMap<String, Vec<RequestData>>) -> Vec<TreeItem<'_, usize>> {
    let mut items = Vec::new();
    let mut counter = 1;

    for key in map.keys() {
        let mut children = Vec::new();
        if let Some(list) = map.get(key) {
            for req in list {
                children.push(TreeItem::new_leaf(counter, req.url.clone()));
                counter += 1;
            }
        }
        counter += 1;
        items.push(TreeItem::new(counter, key.clone(), children).expect("error"));
    }

    items
}
