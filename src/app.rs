use std::collections::HashMap;

use reqwest::Method;

use serde::{Deserialize, Serialize};

use crate::data::http_method_serde;

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
}

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    #[serde(with = "http_method_serde")]
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
}

impl App {
    pub fn new() -> App {
        App {
            requestCollections: HashMap::new(),
            currentRequest: None,
            currentView: CurrentView::Overview,
        }
    }
}
