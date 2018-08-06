///DTO for the endpoints returning multiple cards
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum CardsDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Cards {
        cards: Vec<CardDetail>,
    },
}

///DTO for the endpoints returning a single cards
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum CardDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Card {
        card: CardDetail,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardDetail {
    pub name: String,
    #[serde(default)]
    pub names: Vec<String>,
    pub mana_cost: Option<String>,
    pub cmc: f64,
    #[serde(default)]
    pub colors: Vec<String>,
    #[serde(default)]
    pub color_identity: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub supertypes: Vec<String>,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub subtypes: Vec<String>,
    pub rarity: String,
    pub set: String,
    pub set_name: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub flavor: String,
    pub artist: String,
    pub number: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub layout: Option<String>,
    pub loyalty: Option<u32>,
    pub multiverseid: Option<u32>,
    pub image_url: Option<String>,
    #[serde(default)]
    pub rulings: Vec<Ruling>,
    pub watermark: Option<String>,
    pub release_date: Option<String>,
    #[serde(default)]
    pub foreign_names: Vec<ForeignName>,
    #[serde(default)]
    pub printings: Vec<String>,
    pub original_text: Option<String>,
    pub original_type: Option<String>,
    #[serde(default)]
    pub legalities: Vec<Legality>,
    #[serde(default)]
    pub variations: Vec<u32>,
    pub border: Option<String>,
    #[serde(default)]
    pub timeshifted: bool,
    pub hand: Option<i32>,
    pub life: Option<i32>,
    #[serde(default)]
    pub reserved: bool,
    #[serde(default)]
    pub starter: bool,
    pub source: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ruling {
    pub date: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Legality {
    pub format: String,
    pub legality: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ForeignName {
    pub image_url: Option<String>,
    pub name: String,
    pub multiverseid: Option<u32>,
    pub language: String,
}
