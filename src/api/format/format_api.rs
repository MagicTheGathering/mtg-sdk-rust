use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use std::sync::Weak;

use API_URL;
use model::format::FormatDto;
use api::response::ApiResponse;

pub struct FormatApi{
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
        let mut response;
        {
            let client = match self.client.upgrade() {
                Some(client) => Ok(client),
                None => Err(MtgIoErrorKind::ClientDropped),
            }?;
            response = client
                .get(&all_url)
                .send()
                .context(MtgIoErrorKind::HttpError)?;
        }
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let formats = serde_json::from_str::<FormatDto>(&body)
            .context(MtgIoErrorKind::FormatBodyParseError)?
            .formats;
        Ok(
            ApiResponse::new(formats, response.headers())
        )
    }
}