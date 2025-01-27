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

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub id: String,
    #[serde(with = "http_method")]
    pub method: Method,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCollection {
    pub id: String,
    pub name: String,
    pub requests: Vec<RequestData>,
}

impl RequestCollection {
    pub fn new(name: &str) -> Self {
        RequestCollection {
            id: App::generate_id(),
            name: name.to_string(),
            requests: vec![],
        }
    }
}

impl RequestData {
    pub fn new() -> Self {
        RequestData {
            id: App::generate_id(),
            method: Method::GET,
            url: "http://google.com".to_string(),
        }
    }
}

pub struct App {
    pub request_collections: Vec<RequestCollection>,
    pub current_request: Option<String>,
    pub editing_state: EditingState,
    pub focused_section: FocusedSection,
    pub request_tree_state: TreeState<String>,
}

impl App {
    pub fn new() -> Self {
        let mut collections = vec![
            RequestCollection::new("Test 1"),
            RequestCollection::new("Test 2"),
        ];
        collections[0].requests.push(RequestData::new());
        let data = App {
            request_collections: collections,
            current_request: None,
            editing_state: EditingState::Nothing,
            focused_section: FocusedSection::Left,
            request_tree_state: TreeState::default(),
        };

        return data;
    }

    pub fn get_tree_state(&mut self) -> &mut TreeState<String> {
        &mut self.request_tree_state
    }

    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn find_request(&self, id: &String) -> Option<&RequestData> {
        self.request_collections
            .iter()
            .flat_map(|col| &col.requests)
            .find(|req| req.id.eq(id))
    }

    pub fn find_collection(&self, id: &String) -> Option<&RequestCollection> {
        self.request_collections.iter().find(|c| c.id.eq(id))
    }

    pub fn select_request(&mut self, id: &str) {
        if self.find_request(&id.to_string()).is_some() {
            self.current_request = Some(id.to_string());
        } else {
            self.current_request = None;
        }
    }

    pub fn get_current_request(&self) -> Option<&RequestData> {
        self.current_request
            .as_deref()
            .and_then(|id| self.find_request(&id.to_string()))
    }
}
