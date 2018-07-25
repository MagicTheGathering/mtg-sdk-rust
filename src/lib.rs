#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate failure_derive;

extern crate failure;
extern crate itertools;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub use api::card::card_api::CardApi as cards;
use api::card::card_api::CardApi;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

pub mod api;
pub mod model;

pub mod prelude {
    pub use api::card::filter::*;
    pub use api::card::filtertypes::*;
    pub use MtgSdk;
}

/// The MTG.io SDK, use this to access the various api calls
#[allow(dead_code)]
pub struct MtgSdk {
    client: Arc<Client>,
    cards: CardApi,
}

impl MtgSdk {
    /// Creates a new MTG.io SDK Struct
    pub fn new(timeout: u64) -> MtgSdk {
        let client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .unwrap(),
        );
        let cards = CardApi::new(Arc::downgrade(&client));

        MtgSdk { client, cards }
    }

    pub fn cards(&self) -> &CardApi {
        &self.cards
    }
}
