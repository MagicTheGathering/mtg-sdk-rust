#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub(crate) enum FormatDto {
    Error {
        status: Option<String>,
        error: String,
    },
    Formats {
        formats: Vec<String>,
    },
}
