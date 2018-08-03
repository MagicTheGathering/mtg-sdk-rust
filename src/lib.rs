#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate failure_derive;

extern crate chrono;
extern crate failure;
extern crate itertools;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub use api::card::card_api::CardApi as cards;
use api::card::card_api::CardApi;
use api::format::format_api::FormatApi;
use api::set::set_api::SetApi;
use api::types::type_api::SubtypeApi;
use api::types::type_api::SupertypeApi;
use api::types::type_api::TypeApi;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

pub mod api;
pub mod model;

pub mod prelude {
    pub use api::card::filter::*;
    pub use api::card::filtertypes::*;
    pub use api::set::filter::*;
    pub use api::set::filtertypes::*;
    pub use MtgClient;
}

const API_URL: &str = "https://api.magicthegathering.io/v1";

/// The MTG.io SDK, use this to access the various api calls
#[allow(dead_code)]
pub struct MtgClient {
    client: Arc<Client>,
    pub cards: CardApi,
    pub sets: SetApi,
    pub types: TypeApi,
    pub subtypes: SubtypeApi,
    pub supertypes: SupertypeApi,
    pub formats: FormatApi,
}

impl MtgClient {
    /// Creates a new MTG.io SDK Struct
    pub fn new(timeout: u64) -> MtgClient {
        let client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .unwrap(),
        );
        let cards = CardApi::new(Arc::downgrade(&client));
        let sets = SetApi::new(Arc::downgrade(&client));
        let types = TypeApi::new(Arc::downgrade(&client));
        let subtypes = SubtypeApi::new(Arc::downgrade(&client));
        let supertypes = SupertypeApi::new(Arc::downgrade(&client));
        let formats = FormatApi::new(Arc::downgrade(&client));

        MtgClient {
            client,
            cards,
            sets,
            types,
            subtypes,
            supertypes,
            formats,
        }
    }

    pub fn cards(&self) -> &CardApi {
        &self.cards
    }
    pub fn sets(&self) -> &SetApi {
        &self.sets
    }
    pub fn types(&self) -> &TypeApi {
        &self.types
    }
    pub fn subtypes(&self) -> &SubtypeApi {
        &self.subtypes
    }
    pub fn supertypes(&self) -> &SupertypeApi {
        &self.supertypes
    }

    pub fn formats(&self) -> &FormatApi {
        &self.formats
    }
}
