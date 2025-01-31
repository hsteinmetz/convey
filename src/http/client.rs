use reqwest::Error;

use crate::app::RequestData;

pub fn request(req: &RequestData) -> Result<reqwest::blocking::Response, Error> {
    reqwest::blocking::get(req.url.clone())
}
