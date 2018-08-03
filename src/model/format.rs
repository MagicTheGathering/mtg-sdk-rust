#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct FormatDto {
    #[serde(default)]
    pub formats: Vec<String>,
}
