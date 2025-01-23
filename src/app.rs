use std::collections::HashMap;

use reqwest::Method;

use serde::{Deserialize, Serialize};

use crate::json::http_method::http_method;

pub enum CurrentView {
    Overview,
    SplitView,
}

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
    pub requestCollections: HashMap<String, Vec<RequestData>>,
    pub currentRequest: Option<RequestData>,
    pub currentView: CurrentView,
    pub editingState: EditingState,
}

impl App {
    pub fn new() -> App {
        App {
            requestCollections: HashMap::new(),
            currentRequest: None,
            currentView: CurrentView::Overview,
            editingState: EditingState::Nothing,
        }
    }
}
