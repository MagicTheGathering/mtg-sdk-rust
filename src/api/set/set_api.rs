use api::error::MtgIoErrorKind;
use api::set::filter::SetFilter;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use model::card::CardDetail;
use model::card::CardsDto;
use model::set::SetDetail;
use model::set::SetDto;
use model::set::SetsDto;
use std::sync::Weak;

use api::response::ApiResponse;
use API_URL;

///Responsible for the calls to the /cards endpoint
pub struct SetApi {
    client: Weak<Client>,
}

impl SetApi {
    pub(crate) fn new(client: Weak<Client>) -> SetApi {
        SetApi { client }
    }

    /// Returns all Sets
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<SetDetail>>, Error> {
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
        let sets = match serde_json::from_str::<SetsDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
        {
            SetsDto::Sets { sets } => Ok(sets),
            SetsDto::Error { error, status } => match status {
                Some(status) => Err(MtgIoErrorKind::ApiError {
                    cause: format!("{}: {}", status, error),
                }),
                None => Err(MtgIoErrorKind::ApiError { cause: error }),
            },
        }?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns all sets matching the supplied filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: SetFilter) -> Result<ApiResponse<Vec<SetDetail>>, Error> {
        let all_url = SetApi::create_filtered_url(filter);
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
        let sets = match serde_json::from_str::<SetsDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
        {
            SetsDto::Sets { sets } => Ok(sets),
            SetsDto::Error { error, status } => match status {
                Some(status) => Err(MtgIoErrorKind::ApiError {
                    cause: format!("{}: {}", status, error),
                }),
                None => Err(MtgIoErrorKind::ApiError { cause: error }),
            },
        }?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns the specified set by the set code
    pub fn find<'a, T>(&self, code: T) -> Result<ApiResponse<SetDetail>, Error>
    where
        T: Into<&'a str>,
    {
        let find_url = [API_URL, "/sets/", code.into()].join("");
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
        let set = match serde_json::from_str::<SetDto>(&body)
            .context(MtgIoErrorKind::SetBodyParseError)?
        {
            SetDto::Set { set } => Ok(set),
            SetDto::Error { error, status } => match status {
                Some(status) => Err(MtgIoErrorKind::ApiError {
                    cause: format!("{}: {}", status, error),
                }),
                None => Err(MtgIoErrorKind::ApiError { cause: error }),
            },
        }?;
        Ok(ApiResponse::new(set, response.headers()))
    }

    /// Returns a sample booster pack of cards from the specified set
    pub fn booster<'a, T>(&self, code: T) -> Result<ApiResponse<Vec<CardDetail>>, Error>
    where
        T: Into<&'a str>,
    {
        let booster_url = [API_URL, "/sets/", code.into()].join("");
        let booster_url = [&booster_url, "/booster"].join("");
        let mut response;
        {
            let client = match self.client.upgrade() {
                Some(client) => Ok(client),
                None => Err(MtgIoErrorKind::ClientDropped),
            }?;
            response = client
                .get(&booster_url)
                .send()
                .context(MtgIoErrorKind::HttpError)?;
        }
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let cards = match serde_json::from_str::<CardsDto>(&body)
            .context(MtgIoErrorKind::CardBodyParseError)?
        {
            CardsDto::Cards { cards } => Ok(cards),
            CardsDto::Error { error, status } => match status {
                Some(status) => Err(MtgIoErrorKind::ApiError {
                    cause: format!("{}: {}", status, error),
                }),
                None => Err(MtgIoErrorKind::ApiError { cause: error }),
            },
        }?;
        Ok(ApiResponse::new(cards, response.headers()))
    }

    fn create_filtered_url(filter: SetFilter) -> String {
        let url = [API_URL, "/sets"].join("");
        if filter.0.is_empty() {
            url
        } else {
            [url, filter.0].join("?")
        }
    }
}
