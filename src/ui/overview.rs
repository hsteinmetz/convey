use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::Block,
    Frame,
};
use tui_tree_widget::{Tree, TreeItem};

use crate::app::{App, RequestCollection};

use super::root_layout::create_root_layout;

pub fn render(frame: &mut Frame, state: &mut App) {
    let root_layout = create_root_layout(frame, "Convey");

    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Percentage(80)])
        .split(root_layout[1]);

    render_list(frame, &sections[0], state);
    render_right_box(frame, &sections[1], state);
}

fn render_list(frame: &mut Frame, area: &Rect, app_state: &mut App) {
    let items = construct_tree(&app_state.request_collections);
    let list = Tree::new(&items)
        .expect("all identifiers are unique")
        .block(
            Block::bordered()
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
                if state.get_tree_state().selected().len() == 0 {
                    return;
                }

                let selected_id = state.get_tree_state().selected()[0].clone();

                if state.find_request(&selected_id).is_some() {
                    println!("selecrting req");
                    state.select_request(&selected_id);
                    state.get_tree_state().select(vec![selected_id]);
                } else if state.find_collection(&selected_id).is_some() {
                    println!("selecting collection");
                    state.get_tree_state().toggle(vec![selected_id]);
                }
            }
            _ => {}
        },
        crate::app::FocusedSection::Right => {}
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

fn render_right_box(frame: &mut Frame, area: &Rect, app_state: &mut App) {
    let title: String = {
        if app_state.get_tree_state().selected().len() == 0 {
            return;
        }

        let selected_id = app_state.get_tree_state().selected()[0].clone();
        match app_state.find_request(&selected_id) {
            Some(req) => req.url.clone(),
            None => "Select a Request".to_string(),
        }
    };
    let container = Block::bordered()
        .title(title)
        .title_bottom("Test bottom")
        .style(Style::default());

    frame.render_widget(container, *area);
}
