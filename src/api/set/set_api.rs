use api::error::MtgIoErrorKind;
use api::set::filter::SetFilter;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use std::sync::Weak;
use model::set::SetDetail;
use model::set::SetsDto;
use model::set::SetDto;

const API_URL: &str = "https://api.magicthegathering.io/v1";

///Responsible for the calls to the /cards endpoint
pub struct SetApi {
    client: Weak<Client>,
}

impl SetApi {
    pub(crate) fn new(client: Weak<Client>) -> SetApi {
        SetApi { client }
    }

    /// Returns a Request Object to fetch all cards
    #[allow(dead_code)]
    pub fn all(&self) -> Result<Vec<SetDetail>, Error> {
        let all_url = [API_URL, "/sets"].join("");
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
        Ok(serde_json::from_str::<SetsDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
            .sets)
    }

    /// Returns a Request Object to fetch all cards with a filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: SetFilter) -> Result<Vec<SetDetail>, Error> {
        let all_url = SetApi::create_filtered_url(&filter);
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
        Ok(serde_json::from_str::<SetsDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
            .sets)
    }

    fn create_filtered_url(filter: &SetFilter) -> String {
        let url = [API_URL, "/sets"].join("");
        if filter.0.is_empty() {
            url
        } else {
            [url, filter.0.clone()].join("?")
        }
    }

    /// Returns a specific card by a specific id
    pub fn find(&self, id: u32) -> Result<SetDetail, Error> {
        let find_url = [API_URL, "/sets/", &id.to_string()].join("");
        let mut response;
        {
            let client = match self.client.upgrade() {
                Some(client) => Ok(client),
                None => Err(MtgIoErrorKind::ClientDropped),
            }?;
            response = client
                .get(&find_url)
                .send()
                .context(MtgIoErrorKind::HttpError)?;
        }
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let card_option = serde_json::from_str::<SetDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
            .set;
        Ok(
            match card_option {
                Some(card) => Ok(card),
                None => Err(MtgIoErrorKind::SetNotFound)
            }?
        )
    }
}