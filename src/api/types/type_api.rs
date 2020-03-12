use crate::api::error::MtgApiErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use std::rc::Weak;

use crate::api::response::ApiResponse;
use crate::api::util;

///Responsible for the calls to the /types endpoint
pub struct TypeApi {
    client: Weak<Client>,
    url: String,
}

///Responsible for the calls to the /subtypes endpoint
pub struct SubtypeApi {
    client: Weak<Client>,
    url: String,
}

///Responsible for the calls to the /supertypes endpoint
pub struct SupertypeApi {
    client: Weak<Client>,
    url: String,
}

impl TypeApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> TypeApi {
        TypeApi { client, url }
    }

    /// Returns all types
    #[allow(dead_code)]
    pub async fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = [&self.url, "/types"].join("");
        let mut response = util::send_response(&url, &self.client).await?;
        let headers = std::mem::take(response.headers_mut());
        let body = response.text().await.context(MtgApiErrorKind::BodyReadError)?;
        let types = util::retrieve_types_from_body(&body)?;
        Ok(ApiResponse::new(types, headers))
    }
}

impl SubtypeApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> SubtypeApi {
        SubtypeApi { client, url }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub async fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = [&self.url, "/subtypes"].join("");
        let mut response = util::send_response(&url, &self.client).await?;
        let headers = std::mem::take(response.headers_mut());
        let body = response.text().await.context(MtgApiErrorKind::BodyReadError)?;
        let subtypes = util::retrieve_subtypes_from_body(&body)?;
        Ok(ApiResponse::new(subtypes, headers))
    }
}

impl SupertypeApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> SupertypeApi {
        SupertypeApi { client, url }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub async fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = [&self.url, "/supertypes"].join("");
        let mut response = util::send_response(&url, &self.client).await?;
        let headers = std::mem::take(response.headers_mut());
        let body = response.text().await.context(MtgApiErrorKind::BodyReadError)?;
        let supertypes = util::retrieve_supertypes_from_body(&body)?;
        Ok(ApiResponse::new(supertypes, headers))
    }
}
