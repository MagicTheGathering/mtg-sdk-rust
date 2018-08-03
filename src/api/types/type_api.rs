use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use model::types::TypesDto;
use std::sync::Weak;

use api::response::ApiResponse;
use model::types::SubtypesDto;
use model::types::SupertypesDto;
use reqwest::Response;
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
        let mut response = send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let types = serde_json::from_str::<TypesDto>(&body)
            .context(MtgIoErrorKind::TypeBodyParseError)?
            .types;
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
        let mut response = send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let subtypes = serde_json::from_str::<SubtypesDto>(&body)
            .context(MtgIoErrorKind::SubtypeBodyParseError)?
            .subtypes;
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
        let mut response = send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let supertypes = serde_json::from_str::<SupertypesDto>(&body)
            .context(MtgIoErrorKind::SupertypeBodyParseError)?
            .supertypes;
        Ok(ApiResponse::new(supertypes, response.headers()))
    }
}

fn send_response(all_url: &str, client: &Weak<Client>) -> Result<Response, Error> {
    let client = match client.upgrade() {
        Some(client) => Ok(client),
        None => Err(MtgIoErrorKind::ClientDropped),
    }?;
    Ok(client
        .get(all_url)
        .send()
        .context(MtgIoErrorKind::HttpError)?)
}
