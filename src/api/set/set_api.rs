use crate::api::error::MtgApiErrorKind;
use crate::api::set::filter::SetFilter;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use crate::model::card::CardDetail;
use crate::model::set::SetDetail;
use std::rc::Weak;

use crate::api::response::ApiResponse;
use crate::api::util;

///Responsible for the calls to the /sets endpoint
pub struct SetApi {
    client: Weak<Client>,
    url: String,
}

impl SetApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> SetApi {
        SetApi { client, url }
    }

    /// Returns all Sets
    #[allow(dead_code)]
    pub fn all(&self) -> Result<ApiResponse<Vec<SetDetail>>, Error> {
        let url = [&self.url, "/sets"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let sets = util::retrieve_sets_from_body(&body)?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns all sets matching the supplied filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: SetFilter) -> Result<ApiResponse<Vec<SetDetail>>, Error> {
        let url = SetApi::create_filtered_url(&self.url, filter);
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let sets = util::retrieve_sets_from_body(&body)?;
        Ok(ApiResponse::new(sets, response.headers()))
    }

    /// Returns the specified set by the set code
    pub fn find<'a, T>(&self, code: T) -> Result<ApiResponse<SetDetail>, Error>
    where
        T: Into<&'a str>,
    {
        let url = [&self.url, "/sets/", code.into()].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let set = util::retrieve_set_from_body(&body)?;
        Ok(ApiResponse::new(*set, response.headers()))
    }

    /// Returns a sample booster pack of cards from the specified set
    pub fn booster<'a, T>(&self, code: T) -> Result<ApiResponse<Vec<CardDetail>>, Error>
    where
        T: Into<&'a str>,
    {
        let url = [&self.url, "/sets/", code.into(), "/booster"].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let cards = util::retrieve_cards_from_body(&body)?;
        Ok(ApiResponse::new(cards, response.headers()))
    }

    fn create_filtered_url(api_url: &str, filter: SetFilter) -> String {
        let url = [api_url, "/sets"].join("");
        if filter.0.is_empty() {
            url
        } else {
            [url, filter.0].join("?")
        }
    }
}
