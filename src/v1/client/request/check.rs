use crate::error::Result;
use reqwest::Response;

pub fn check_res(response: Response) -> Result<Response> {
    match response.status() {
        reqwest::StatusCode::OK => {
            Ok(response)
        }
        _ => {
            Err(response.into())
        }
    }
}
