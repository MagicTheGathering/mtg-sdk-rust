use api::error::MtgIoErrorKind;
use api::card::filter::CardFilter;
use api::card::filtertypes::CardResponseField;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;
use serde_json;

use hyper::header::Headers;
use model::card::CardDetail;
use model::card::CardDto;
use model::card::CardsDto;
use std::sync::Weak;

header! { (PageSizeHeader, "Page-Size") => [u32] }
header! { (CountHeader, "Count") => [u32] }
header! { (TotalCountHeader, "Total-Count") => [u32] }
header! { (RatelimitLimitHeader, "Ratelimit-Limit") => [u32] }
header! { (RatelimitRemainingHeader, "Ratelimit-Remaining") => [u32] }

const API_URL: &str = "https://api.magicthegathering.io/v1";

///Responsible for the calls to the /cards endpoint
pub struct CardApi {
    client: Weak<Client>,
}

impl CardApi {
    pub(crate) fn new(client: Weak<Client>) -> CardApi {
        CardApi { client }
    }

    /// Returns a Request Object to fetch all cards
    #[allow(dead_code)]
    pub fn all(&self) -> Box<AllCardsRequest> {
        AllCardsRequest::new(self.client.clone(), 100)
    }

    /// Returns a Request Object to fetch all cards with a filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: CardFilter) -> Box<AllCardsRequest> {
        AllCardsRequest::new_filtered(self.client.clone(), 100, filter)
    }

    /// Returns a specific card by a specific id
    pub fn find(&self, id: u32) -> Result<CardDetail, Error> {
        let find_url = [API_URL, "/cards/", &id.to_string()].join("");
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
        let card_option = serde_json::from_str::<CardDto>(&body)
        .context(MtgIoErrorKind::CardBodyParseError)?
            .card;
        Ok(
            match card_option {
                Some(card) => Ok(card),
                None => Err(MtgIoErrorKind::CardNotFound)
            }?
        )
    }
}

/// Request Object to be used to execute requests to the API
#[allow(dead_code)]
pub struct AllCardsRequest {
    page: u32,
    client: Weak<Client>,
    url: String,
    filter: CardFilter,
    order_by: CardResponseField,
    page_size: u32,
}

impl AllCardsRequest {
    fn new(client: Weak<Client>, page_size: u32) -> Box<AllCardsRequest> {
        let url = [API_URL, "cards"].join("/");
        Box::new(AllCardsRequest {
            page: 1,
            client,
            url,
            filter: CardFilter(String::new()),
            page_size,
            order_by: CardResponseField::Name,
        })
    }

    fn new_filtered(
        client: Weak<Client>,
        page_size: u32,
        filter: CardFilter,
    ) -> Box<AllCardsRequest> {
        let url = [API_URL, "cards"].join("/");
        Box::new(AllCardsRequest {
            page: 1,
            client,
            url,
            filter,
            page_size,
            order_by: CardResponseField::Name,
        })
    }

    /// Executes the call to the API.
    /// Repeated calls to this method will return the different pages of the cards API

    /// ```no_run
    /// # use std::error::Error;
    /// # use mtgio_client::prelude::*;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let sdk = MtgClient::new(60);
    /// let mut get_cards_request = sdk.cards().all();
    /// let mut cards = Vec::new();
    /// loop {
    ///     let response = get_cards_request.next_page()?;
    ///     if response.cards.len() == 0 {break}
    ///     cards.extend(response.cards);
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    /// # Errors
    ///
    /// If this function can't connect to the API or does not manage
    /// to read the response, it will return an error.
    ///
    #[allow(dead_code)]
    pub fn next_page(&mut self) -> Result<AllCardsResponse, Error> {
        let url = self.create_filtered_url();
        let client = match self.client.upgrade() {
            Some(client) => Ok(client),
            None => Err(MtgIoErrorKind::ClientDropped),
        }?;
        println!("{}", url);
        let mut response = client.get(&url).send().context(MtgIoErrorKind::HttpError)?;
        self.page += 1;
        let headers = response.headers().clone();
        let body = response.text().context(MtgIoErrorKind::CardBodyParseError)?;
        Ok(self.create_response(&headers, &body)?)
    }

    /// Sets the ordering of the cards
    #[allow(dead_code)]
    pub fn order_by(&mut self, field: CardResponseField) {
        self.order_by = field;
    }

    /// Sets the page for the following API calls
    #[allow(dead_code)]
    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }

    /// Sets the page size for the following API calls
    #[allow(dead_code)]
    pub fn set_page_size(&mut self, size: u32) {
        self.page_size = size;
    }

    fn create_filtered_url(&self) -> String {
        let page_filter = ["page", self.page.to_string().as_str()].join("=");
        let paged_filter = if self.filter.0.is_empty() {
            [self.filter.0.as_str(), &page_filter].join("")
        } else {
            [self.filter.0.as_str(), &page_filter].join("&")
        };
        let page_size_filter = ["pageSize", &self.page_size.to_string()].join("=");
        let paged_filter_sized = [paged_filter, page_size_filter].join("&");

        [self.url.as_str(), paged_filter_sized.as_str()].join("?")
    }

    fn create_response(&self, headers: &Headers, body: &str) -> Result<AllCardsResponse, Error> {
        let cards = serde_json::from_str::<CardsDto>(body)
            .context(MtgIoErrorKind::CardBodyParseError)?
            .cards;
        let page_size = headers.get::<PageSizeHeader>().map(|header| header.0);
        let count = headers.get::<CountHeader>().map(|header| header.0);
        let total_count = headers.get::<TotalCountHeader>().map(|header| header.0);
        let ratelimit_limit = headers.get::<RatelimitLimitHeader>().map(|header| header.0);
        let ratelimit_remaining = headers
            .get::<RatelimitRemainingHeader>()
            .map(|header| header.0);
        Ok(AllCardsResponse {
            cards,
            page_size,
            count,
            total_count,
            ratelimit_limit,
            ratelimit_remaining,
        })
    }
}

/// Response returned by the Cards API
#[allow(dead_code)]
pub struct AllCardsResponse {
    pub cards: Vec<CardDetail>,
    pub page_size: Option<u32>,
    pub count: Option<u32>,
    pub total_count: Option<u32>,
    pub ratelimit_limit: Option<u32>,
    pub ratelimit_remaining: Option<u32>,
}
