#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct TypesDto {
    #[serde(default)]
    pub types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct SubtypesDto {
    #[serde(default)]
    pub subtypes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct SupertypesDto {
    #[serde(default)]
    pub supertypes: Vec<String>,
}
