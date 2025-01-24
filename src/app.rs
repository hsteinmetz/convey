use std::collections::HashMap;

use ratatui::widgets::ListState;
use reqwest::Method;

use serde::{Deserialize, Serialize};
use tui_tree_widget::TreeState;

use crate::json::http_method::http_method;

pub enum FocusedSection {
    Left,
    Right,
}

#[derive(Debug)]
pub enum EditingState {
    Url,
    Name,
    Body,
    HeaderKey,
    HeaderValue,
    Nothing,
}

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    #[serde(with = "http_method")]
    pub method: Method,
    pub url: String,
}

impl RequestData {
    pub fn new() -> RequestData {
        RequestData {
            method: Method::GET,
            url: "http://google.com".to_string(),
        }
    }
}

pub struct App {
    pub request_collections: HashMap<String, Vec<RequestData>>,
    pub current_request: Option<RequestData>,
    pub editing_state: EditingState,
    pub focused_section: FocusedSection,
    pub request_tree_state: TreeState<usize>,
}

impl App {
    pub fn new() -> App {
        App {
            request_collections: HashMap::new(),
            current_request: None,
            editing_state: EditingState::Nothing,
            focused_section: FocusedSection::Left,
            request_tree_state: TreeState::default(),
        }
    }
}
