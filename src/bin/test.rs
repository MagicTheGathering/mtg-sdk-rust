extern crate mtgio_client;
extern crate serde_json;

use mtgio_client::api::card::filter::CardFilter;
use mtgio_client::api::card::filtertypes::CardRarity;
use mtgio_client::api::card::filtertypes::CardType;
use mtgio_client::api::card::filtertypes::GameFormat;
use mtgio_client::MtgClient;
use std::error::Error;

fn main() {
    try_main().unwrap();
}

fn try_main() -> Result<(), Box<Error>> {
    let mut filtered_cards = Vec::new();

    //Create new MtgClient
    let api = MtgClient::new(100);

    // Create a unfiltered cards request
    let mut get_cards_request = api.cards().all();

    //Start at Page 20
    get_cards_request.set_page(20);

    //collect all cards from the first 5 pages
    for _ in 0..5 {
        let cards = get_cards_request.next_page()?.content;
        if cards.is_empty() {
            break;
        }
        filtered_cards.extend(cards);
    }
    println!("Unfiltered Cards: {:?}", filtered_cards);

    // Create a filtered cards request
    let mut get_cards_request = api.cards().all_filtered(
        CardFilter::builder()
            .game_format(GameFormat::Standard)
            .cardtypes_or(&[CardType::Instant, CardType::Sorcery])
            .converted_mana_cost(2)
            .rarities(&[CardRarity::Rare, CardRarity::MythicRare])
            .build(),
    );

    //collect all cards filtered from the first 5 pages
    loop {
        let cards = get_cards_request.next_page()?.content;
        if cards.is_empty() {
            break;
        }
        filtered_cards.extend(cards);
    }
    println!("Filtered Cards: {:?}", filtered_cards);

    let sets = api.sets().booster("ktk")?.content;
    println!("Random Cards from Booster: {:?}", sets);
    let types = api.types().all()?.content;
    println!("Types: {:?}", types);
    let subtypes = api.subtypes().all()?.content;
    println!("Subtypes: {:?}", subtypes);
    let supertypes = api.supertypes().all()?.content;
    println!("Supertypes: {:?}", supertypes);
    let formats = api.formats().all()?.content;
    println!("Formats: {:?}", formats);
    Ok(())
}
