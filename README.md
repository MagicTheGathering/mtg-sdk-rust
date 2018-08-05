# mtgio-sdk
Rust Wrapper around the https://magicthegathering.io/ API

In-depth documentation about the capabilities can be found here: https://docs.magicthegathering.io/

This Wrapper supplies the related methods to call the API and the conversion of the supplied json data into rust objects.

### Integration
There is no way to retrieve this library using crates.io yet, will follow soon. 
```
# Cargo.toml
[dependencies]
mtgio-client = { git = "https://github.com/AlexW-GH/mtgio-client" }
```

```
# main.rs / lib.rs
extern crate mtgio_client;
```

### Documentation

~ Docs will follow after the crate has been published, you can build them locally by cloning and using 'cargo doc'

### Usage

```
// Create a client with the specified (per request) timeout 
let api = MtgClient::new(100);
```

##### Example: Get all Cards on pages 20 - 25
The Page size for cards requests is 100 cards by default.

```
// Create a unfiltered cards request
let mut get_cards_request = api.cards().all();

//Start at Page 20
get_cards_request.set_page(20);

//collect all cards from pages 20 to 25
for _ in 0..5 {
    let cards = get_cards_request.next_page()?.content;
    if cards.is_empty() {
        break;
    }
    filtered_cards.extend(cards);
}
println!("Unfiltered Cards: {:?}", filtered_cards);
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
