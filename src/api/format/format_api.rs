use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use std::sync::Weak;

use api::response::ApiResponse;
use model::format::FormatDto;
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
        let formats = match serde_json::from_str::<FormatDto>(&body)
            .context(MtgIoErrorKind::FormatBodyParseError)?
            {
                FormatDto::Formats {formats} => Ok(formats),
                FormatDto::Error{error, status} => {
                    match status {
                        Some(status) => Err(MtgIoErrorKind::ApiError {cause: format!("{}: {}", status, error)}),
                        None => Err(MtgIoErrorKind::ApiError {cause: error})
                    }
                }
            }?;
        Ok(ApiResponse::new(formats, response.headers()))
    }
}
