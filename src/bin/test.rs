extern crate mtgio_client;
extern crate serde_json;

use mtgio_client::MtgClient;
use serde_json::Error;

fn main() {
    println!("before main");
    try_main().unwrap();
    println!("after main");
}

fn try_main() -> Result<(), Error> {
    let sdk = MtgClient::new(100);
    /*let mut get_cards_request = sdk.cards().all();
    get_cards_request.set_page_size(100);
    get_cards_request.set_page(4);
    let mut cards = Vec::new();
    let mut page = 1;
    loop {
        let response = get_cards_request.next_page();
        match response {
            Ok(resp) => {
                if resp.cards.len() == 0 {break}
                println!("Page {}, Requests left: {:?}", page, resp.ratelimit_remaining.map(|val| val.to_string()));
                cards.extend(resp.cards);
            }
            Err(err) => {
                println!("Page {}, Error: {}", page, err);
                panic!(err);
            }
        }
        page += 1;
    }*/
    let sets = sdk.sets().all();
    println!("{:?}", sets);
    Ok(())
}
