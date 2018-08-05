use api::error::MtgIoErrorKind;
use api::set::filter::SetFilter;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use model::card::CardDetail;
use model::set::SetDetail;
use std::sync::Weak;

use api::response::ApiResponse;
use api::util;
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
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let sets = util::retrieve_sets_from_body(&body)?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns all sets matching the supplied filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: SetFilter) -> Result<ApiResponse<Vec<SetDetail>>, Error> {
        let all_url = SetApi::create_filtered_url(filter);
        let mut response = util::send_response(&all_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let sets = util::retrieve_sets_from_body(&body)?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns the specified set by the set code
    pub fn find<'a, T>(&self, code: T) -> Result<ApiResponse<SetDetail>, Error>
    where
        T: Into<&'a str>,
    {
        let find_url = [API_URL, "/sets/", code.into()].join("");
        let mut response = util::send_response(&find_url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let set = util::retrieve_set_from_body(&body)?;
        Ok(ApiResponse::new(set, response.headers()))
    }

    /// Returns a sample booster pack of cards from the specified set
    pub fn booster<'a, T>(&self, code: T) -> Result<ApiResponse<Vec<CardDetail>>, Error>
    where
        T: Into<&'a str>,
    {
        let url = [API_URL, "/sets/", code.into()].join("");
        let url = [&url, "/booster"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgIoErrorKind::BodyReadError)?;
        let cards = util::retrieve_cards_from_body(&body)?;
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
