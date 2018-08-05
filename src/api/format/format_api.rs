use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use std::sync::Weak;

use api::response::ApiResponse;
use api::util;
use API_URL;

pub struct FormatApi {
    client: Weak<Client>,
}

impl FormatApi {
    pub(crate) fn new(client: Weak<Client>) -> FormatApi {
        FormatApi { client }
    }

    /// Returns all types
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let all_url = [API_URL, "/formats"].join("");
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let formats = util::retrieve_formats_from_body(&body)?;
        Ok(ApiResponse::new(formats, response.headers()))
    }
}
