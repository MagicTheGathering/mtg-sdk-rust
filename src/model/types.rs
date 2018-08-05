#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum TypesDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Types {
        types: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum SubtypesDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Subtypes {
        subtypes: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum SupertypesDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Supertypes {
        supertypes: Vec<String>,
    },
}
