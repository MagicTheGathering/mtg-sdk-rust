use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum SetsDto {
    Error {
        status: Option<String>,
        error: String
    },
    Sets {
        sets: Vec<SetDetail>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum SetDto {
    Error {
        status: Option<String>,
        error: String
    },
    Set {
        set: SetDetail,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetDetail {
    pub code: String,
    pub name: String,
    #[serde(rename = "types")]
    pub type_: String,
    pub block: Option<String>,
    pub gatherer_code: Option<String>,
    pub old_code: Option<String>,
    pub magic_cards_info_code: Option<String>,
    pub release_date: NaiveDate,
    pub border: String,
    pub expansion: Option<String>,
    pub online_only: Option<bool>,
    #[serde(default)]
    pub booster: Vec<Booster>,
    #[serde(rename = "mkm_name")]
    pub mkm_name: Option<String>,
    #[serde(rename = "mkm_id")]
    pub mkm_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Booster {
    Multiple(Vec<String>),
    Single(String),
}
