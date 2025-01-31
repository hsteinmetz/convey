use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use reqwest::{Method, Url};

use crate::{
    app::{App, RequestData},
    http,
};

pub fn render(frame: &mut Frame, area: &Rect, app: &mut App) {
    let title: String = {
        if app.get_tree_state().selected().len() == 0 {
            "Select a Request".to_string()
        } else {
            let selected_id = app.get_tree_state().selected()[0].clone();
            match app.find_request(&selected_id) {
                Some(req) => req.url.clone(),
                None => "Select a Request".to_string(),
            }
        }
    };
    let container = Block::new()
        .borders(Borders::RIGHT | Borders::LEFT)
        .border_style(match app.focused_section {
            crate::app::FocusedSection::Right => Style::default(),
            crate::app::FocusedSection::Left => Style::new().dark_gray(),
        })
        .title(title)
        .title_bottom("Test bottom")
        .style(Style::default());

    let paragraph = Paragraph::new(app.request_window_state.response_text.clone())
        .wrap(Wrap { trim: false })
        .scroll((app.request_window_state.response_scroll_pos, 0))
        .block(container);

    frame.render_widget(paragraph, *area);
}

pub fn handle_key(key: KeyEvent, state: &mut App) {
    match key.code {
        KeyCode::Enter => match send_and_handle() {
            Ok(result) => {
                state.request_window_state.response_text = result;
            }

            Err(_) => (),
        },
        KeyCode::Char('j') => {
            state.request_window_state.response_scroll_pos = state
                .request_window_state
                .response_scroll_pos
                .saturating_add(1);
        }
        KeyCode::Char('k') => {
            state.request_window_state.response_scroll_pos = state
                .request_window_state
                .response_scroll_pos
                .saturating_sub(1);
        }
        _ => (),
    }
}

fn send_and_handle() -> Result<String, reqwest::Error> {
    let req = RequestData {
        id: "1".to_string(),
        method: Method::GET,
        url: Url::parse("http://google.com").unwrap().to_string(),
    };
    http::client::request(&req)?.text()
}
