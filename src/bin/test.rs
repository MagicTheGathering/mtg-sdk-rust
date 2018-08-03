extern crate mtgio_client;
extern crate serde_json;

use mtgio_client::MtgClient;
use std::error::Error;
use mtgio_client::api::card::filter::CardFilter;
use mtgio_client::api::card::filtertypes::CardType;
use mtgio_client::api::card::filtertypes::CardColor;
use mtgio_client::api::card::filtertypes::GameFormat;

fn main() {
    try_main().unwrap();
}

fn try_main() -> Result<(), Box<Error>> {
    let mut filtered_cards = Vec::new();

    //Create new MtgClient
    let api = MtgClient::new(100);

    // Create a filtered cards request
    let mut get_cards_request = api.cards()
        .all_filtered(
            CardFilter::builder()
                .cardtype(CardType::Instant)
                .colors_or(&[CardColor::Blue, CardColor::Red])
                .game_format(GameFormat::Standard)
                .converted_mana_cost(2)
                .build());

    //Return 2 cards per request
    get_cards_request.set_page_size(2);

    //collect all cards from the first 5 pages
    for _ in 0..5 {
        let cards = get_cards_request.next_page()?.content;
        if cards.len() == 0 {
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
