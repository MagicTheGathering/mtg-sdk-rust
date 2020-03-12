use crate::api::error::MtgApiErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use std::rc::Weak;

use crate::api::response::ApiResponse;
use crate::api::util;

///Responsible for the calls to the /formats endpoint
pub struct FormatApi {
    client: Weak<Client>,
    url: String,
}

impl FormatApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> FormatApi {
        FormatApi { client, url }
    }

    /// Returns all types
    #[allow(dead_code)]
    pub async fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = [&self.url, "/formats"].join("");
        let mut response = util::send_response(&url, &self.client).await?;
        let headers = std::mem::take(response.headers_mut());
        let body = response.text().await.context(MtgApiErrorKind::BodyReadError)?;
        let formats = util::retrieve_formats_from_body(&body)?;
        Ok(ApiResponse::new(formats, headers))
    }
}
