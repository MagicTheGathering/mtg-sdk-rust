use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use std::sync::Weak;
use model::types::TypesDto;

use API_URL;
use model::types::SubtypesDto;
use model::types::SupertypesDto;

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
    pub fn all(&self) -> Result<Vec<String>, Error> {
        let all_url = [API_URL, "/types"].join("");
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
        Ok(serde_json::from_str::<TypesDto>(&body)
            .context(MtgIoErrorKind::TypeBodyParseError)?
            .types)
    }
}

impl SubtypeApi {
    pub(crate) fn new(client: Weak<Client>) -> SubtypeApi {
        SubtypeApi { client }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub fn all(&self) -> Result<Vec<String>, Error> {
        let all_url = [API_URL, "/subtypes"].join("");
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
        Ok(serde_json::from_str::<SubtypesDto>(&body)
            .context(MtgIoErrorKind::SubtypeBodyParseError)?
            .subtypes)
    }
}

impl SupertypeApi {
    pub(crate) fn new(client: Weak<Client>) -> SupertypeApi {
        SupertypeApi { client }
    }

    /// Returns all subtypes
    #[allow(dead_code)]
    pub fn all(&self) -> Result<Vec<String>, Error> {
        let all_url = [API_URL, "/supertypes"].join("");
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
        Ok(serde_json::from_str::<SupertypesDto>(&body)
            .context(MtgIoErrorKind::SupertypeBodyParseError)?
            .supertypes)
    }
}