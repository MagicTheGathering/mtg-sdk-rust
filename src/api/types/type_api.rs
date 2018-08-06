use api::error::MtgApiErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use std::sync::Weak;

use api::response::ApiResponse;
use api::util;
use API_URL;

///Responsible for the calls to the /types endpoint
pub struct TypeApi {
    client: Weak<Client>,
}

///Responsible for the calls to the /subtypes endpoint
pub struct SubtypeApi {
    client: Weak<Client>,
}

///Responsible for the calls to the /supertypes endpoint
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
        let url = [API_URL, "/types"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
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
        let url = [API_URL, "/subtypes"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
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
        let url = [API_URL, "/supertypes"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let supertypes = util::retrieve_supertypes_from_body(&body)?;
        Ok(ApiResponse::new(supertypes, response.headers()))
    }
}
