#[macro_use]
extern crate serde_derive;
extern crate hyper_rustls;
extern crate failure_derive;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate failure;
extern crate itertools;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub use crate::api::card::card_api::CardApi as cards;
use crate::api::card::card_api::CardApi;
use crate::api::format::format_api::FormatApi;
use crate::api::set::set_api::SetApi;
use crate::api::types::type_api::SubtypeApi;
use crate::api::types::type_api::SupertypeApi;
use crate::api::types::type_api::TypeApi;
use reqwest::Client;
use std::time::Duration;
use std::rc::Rc;

pub mod api;
pub mod model;

pub mod prelude {
    pub use crate::api::card::filter::*;
    pub use crate::api::card::filtertypes::*;
    pub use crate::api::set::filter::*;
    pub use crate::api::set::filtertypes::*;
    pub use crate::MtgClient;
}

//const API_URL: &str = "https://api.magicthegathering.io/v1";

/// The MTG.io SDK, use this to access the various api calls
#[allow(dead_code)]
pub struct MtgClient {
    client: Rc<Client>,
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
        Self::new_with_url("https://api.magicthegathering.io/v1", timeout)
    }

    /// Creates a new MTG.io SDK Struct with an alternate URL
    /// "https://api.magicthegathering.io/v1" is the default
    pub fn new_with_url(url: &str, timeout: u64) -> MtgClient {
        let client = Rc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .unwrap(),
        );
        let cards = CardApi::new(Rc::downgrade(&client), url.to_string());
        let sets = SetApi::new(Rc::downgrade(&client), url.to_string());
        let types = TypeApi::new(Rc::downgrade(&client), url.to_string());
        let subtypes = SubtypeApi::new(Rc::downgrade(&client), url.to_string());
        let supertypes = SupertypeApi::new(Rc::downgrade(&client), url.to_string());
        let formats = FormatApi::new(Rc::downgrade(&client), url.to_string());

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

    /// /cards enpoint
    pub fn cards(&self) -> &CardApi {
        &self.cards
    }

    /// /sets enpoint
    pub fn sets(&self) -> &SetApi {
        &self.sets
    }

    /// /types enpoint
    pub fn types(&self) -> &TypeApi {
        &self.types
    }

    /// /subtypes enpoint
    pub fn subtypes(&self) -> &SubtypeApi {
        &self.subtypes
    }

    /// /supertypes enpoint
    pub fn supertypes(&self) -> &SupertypeApi {
        &self.supertypes
    }

    /// /formats enpoint
    pub fn formats(&self) -> &FormatApi {
        &self.formats
    }
}
