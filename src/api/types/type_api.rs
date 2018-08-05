use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use std::sync::Weak;

use api::response::ApiResponse;
use api::util;
use API_URL;

pub struct TypeApi {
    client: Weak<Client>,
}

pub struct SubtypeApi {
    client: Weak<Client>,
}

pub struct SupertypeApi {
    client: Weak<Client>,
}

impl TypeApi {
    pub(crate) fn new(client: Weak<Client>) -> TypeApi {
        TypeApi { client }
    }

    /// Returns all types
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let all_url = [API_URL, "/types"].join("");
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let types = util::retrieve_types_from_body(&body)?;
        Ok(ApiResponse::new(types, response.headers()))
    }
}

impl SubtypeApi {
    pub(crate) fn new(client: Weak<Client>) -> SubtypeApi {
        SubtypeApi { client }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let all_url = [API_URL, "/subtypes"].join("");
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let subtypes = util::retrieve_subtypes_from_body(&body)?;
        Ok(ApiResponse::new(subtypes, response.headers()))
    }
}

impl SupertypeApi {
    pub(crate) fn new(client: Weak<Client>) -> SupertypeApi {
        SupertypeApi { client }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<String>>, Error> {
        let all_url = [API_URL, "/supertypes"].join("");
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let supertypes = util::retrieve_supertypes_from_body(&body)?;
        Ok(ApiResponse::new(supertypes, response.headers()))
    }
}
