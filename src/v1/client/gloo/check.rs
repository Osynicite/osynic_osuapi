use crate::error::Result;
use gloo_net::http::Response;

pub fn check_res(response: Response) -> Result<Response> {
    if response.ok() {
        return Ok(response);
    } else {
        return Err(response.into());
    }
}