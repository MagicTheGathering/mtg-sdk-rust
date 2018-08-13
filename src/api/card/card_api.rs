use api::card::filter::CardFilter;
use api::card::filtertypes::CardResponseField;
use api::error::MtgApiErrorKind;
use failure::Error;
use failure::ResultExt;
use reqwest::Client;

use api::response::ApiResponse;
use model::card::CardDetail;
use std::rc::Weak;

use api::util;

///Responsible for the calls to the /cards endpoint
pub struct CardApi {
    client: Weak<Client>,
    url: String,
}

impl CardApi {
    pub(crate) fn new(client: Weak<Client>, url: String) -> CardApi {
        CardApi { client, url }
    }

    /// Returns a Request Object to fetch all cards
    #[allow(dead_code)]
    pub fn all(&self) -> Box<AllCardsRequest> {
        AllCardsRequest::new(self.client.clone(), &self.url, 100)
    }

    /// Returns a Request Object to fetch all cards with a filter
    #[allow(dead_code)]
    pub fn all_filtered(&self, filter: CardFilter) -> Box<AllCardsRequest> {
        AllCardsRequest::new_filtered(self.client.clone(), &self.url, 100, filter)
    }

    /// Returns a specific card by a specific id
    pub fn find(&self, id: u32) -> Result<ApiResponse<CardDetail>, Error> {
        let url = [&self.url, "/cards/", &id.to_string()].join("");
        let mut response = util::send_response(&url, &self.client)?;
        let body = response.text().context(MtgApiErrorKind::BodyReadError)?;
        let card = util::retrieve_card_from_body(&body)?;
        Ok(ApiResponse::new(card, response.headers()))
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
    fn new(client: Weak<Client>, api_url: &str, page_size: u32) -> Box<AllCardsRequest> {
        let url = [api_url, "cards"].join("/");
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
        api_url: &str,
        page_size: u32,
        filter: CardFilter,
    ) -> Box<AllCardsRequest> {
        let url = [api_url, "cards"].join("/");
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
    /// # use mtgapi_client::prelude::*;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let sdk = MtgClient::new(60);
    /// let mut get_cards_request = sdk.cards().all();
    /// let mut cards = Vec::new();
    /// loop {
    ///     let response = get_cards_request.next_page()?;
    ///     if response.content.is_empty() {break}
    ///     cards.extend(response.content);
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
    pub fn next_page(&mut self) -> Result<ApiResponse<Vec<CardDetail>>, Error> {
        let url = self.create_filtered_url();
        let mut response = util::send_response(&url, &self.client)?;
        self.page += 1;
        let headers = response.headers().clone();
        let body = response.text().context(MtgApiErrorKind::CardBodyParseError)?;
        let cards = util::retrieve_cards_from_body(&body)?;
        Ok(ApiResponse::new(cards, &headers))
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
}
