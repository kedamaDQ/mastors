use reqwest::blocking::Response;
use crate::{
    Error,
    Result,
    error::ReceivedMessage,
};

const EXPECTED_CONTENT_TYPE: &str = "application/json";

pub(crate) fn extract_response(resp: Response) -> Result<Response> {
    let status = resp.status();
    let url = resp.url().clone();

    if status.is_success() {
        Ok(resp)
    } else if status.is_client_error() {
        if let Some(content_type) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
            if content_type.to_str().unwrap_or("").starts_with(EXPECTED_CONTENT_TYPE) {
                Err(
                    Error::HttpClientStatusError(url, status.as_u16(), resp.json::<ReceivedMessage>()?)
                )
            } else {
                Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
            }
        } else {
            Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
        }
    } else if status.is_server_error() {
        Err(Error::HttpServerStatusError(url, status.as_u16()))
    } else {
        Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
    }
}
