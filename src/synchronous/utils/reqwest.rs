use reqwest::blocking::Response;
use crate::{
    Error,
    Result,
    error::ReceivedMessage,
};

const EXPECTED_CONTENT_TYPE: &str = "application/json";

pub(crate) fn check_response(resp: Response) -> Result<Response> {
    let status = resp.status();
    let url = resp.url().clone();

    if status.is_success() {
        Ok(resp)
    } else if status.is_client_error() {
        if let Some(content_type) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
            if content_type.to_str().unwrap_or("").starts_with(EXPECTED_CONTENT_TYPE) {
               return Err(
                    Error::HttpClientStatusError(url, status.as_u16(), Box::new(resp.json::<ReceivedMessage>()?))
                );
            }
            Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
        } else {
            Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
        }
    } else if status.is_server_error() {
        Err(Error::HttpServerStatusError(url, status.as_u16()))
    } else {
        Err(Error::HttpUnexpectedStatusError(url, status.as_u16()))
    }
}

pub(crate) fn build_array_query<'a, T>(key: &'a str, values: &'a [T]) -> Vec<(&'a str, &'a str)>
where
    T: AsRef<str> + 'a,
{
    let mut array_query: Vec<(&'a str, &'a str)> = Vec::new();
    for value in values {
        array_query.push((key, value.as_ref()));
    }
    array_query
}
