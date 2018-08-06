# MTG-API Client
Rust Wrapper around the https://magicthegathering.io/ API

In-depth documentation about the capabilities can be found here: https://docs.magicthegathering.io/

This Wrapper supplies the related methods to call the API and the conversion of the supplied json data into rust objects.

### Documentation
Documentation can be found on docs.rs: https://docs.rs/mtgapi-client

### Integration
```
# Cargo.toml
[dependencies]
mtgapi-client = "0.1"
```

```
# main.rs / lib.rs
extern crate mtgapi_client;
```

### Usage

```
// Create a client with the specified (per request) timeout 
use mtgapi_client::MtgClient;
let api = MtgClient::new(100);
```

##### Example: Get all Cards on pages 20 - 25
The Page size for cards requests is 100 cards by default.

```
// Create a unfiltered cards request
let mut get_cards_request = api.cards().all();

//Start at Page 20
get_cards_request.set_page(20);

let mut cards: Vec<CardDetail> = Vec::new();
//collect all cards from pages 20 to 25
for _ in 0..5 {
    let cards = get_cards_request.next_page()?.content;
    if cards.is_empty() {
        break;
    }
    unfiltered_cards.extend(cards);
}
println!("Unfiltered Cards: {:?}", unfiltered_cards);
```

##### Example: Get all cards matching a filter

```
// Create a filtered cards request
let mut get_cards_request = api.cards().all_filtered(
    CardFilter::builder()
        .game_format(GameFormat::Standard)
        .cardtypes_or(&[CardType::Instant, CardType::Sorcery])
        .converted_mana_cost(2)
        .rarities(&[CardRarity::Rare, CardRarity::MythicRare])
        .build(),
    );
    
let mut cards: Vec<CardDetail> = Vec::new();
//collect all cards matching the filter
loop {
    let cards = get_cards_request.next_page()?.content;
    if cards.is_empty() {
        break;
    }
    filtered_cards.extend(cards);
}
println!("Filtered Cards: {:?}", filtered_cards);
```

##### Some example API-calls

```
// Get all sets
let sets = api.sets().all()?.content;
println!("All Sets: {:?}", sets);

// Get an example booster pack from the specified set
let booster_cards = api.sets().booster("ktk")?.content;
println!("Random Cards from Booster: {:?}", booster_cards);

// Get all card types
let types = api.types().all()?.content;
println!("Types: {:?}", types);

// Get all card subtypes
let subtypes = api.subtypes().all()?.content;
println!("Subtypes: {:?}", subtypes);

// Get all card supertypes
let supertypes = api.supertypes().all()?.content;
println!("Supertypes: {:?}", supertypes);

// Get all game formats
let formats = api.formats().all()?.content;
println!("Formats: {:?}", formats);
```
