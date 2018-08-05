use api::error::MtgIoErrorKind;
use failure::Error;
use failure::ResultExt;
use model::card::CardDetail;
use model::set::SetDetail;
use reqwest::Client;
use reqwest::Response;
use serde_json;
use std::sync::Weak;

pub(crate) fn send_response(url: &str, client: &Weak<Client>) -> Result<Response, Error> {
    let client = match client.upgrade() {
        Some(client) => Ok(client),
        None => Err(MtgIoErrorKind::ClientDropped),
    }?;
    info!("GET; {}", &url);
    Ok(client.get(url).send().context(MtgIoErrorKind::HttpError)?)
}

pub(crate) fn retrieve_cards_from_body(body: &str) -> Result<Vec<CardDetail>, Error> {
    use model::card::CardsDto;
    match serde_json::from_str::<CardsDto>(&body).context(MtgIoErrorKind::CardBodyParseError)? {
        CardsDto::Cards { cards } => Ok(cards),
        CardsDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_card_from_body(body: &str) -> Result<CardDetail, Error> {
    use model::card::CardDto;
    match serde_json::from_str::<CardDto>(&body).context(MtgIoErrorKind::CardBodyParseError)? {
        CardDto::Card { card } => Ok(card),
        CardDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_sets_from_body(body: &str) -> Result<Vec<SetDetail>, Error> {
    use model::set::SetsDto;
    match serde_json::from_str::<SetsDto>(&body).context(MtgIoErrorKind::SetBodyParseError)? {
        SetsDto::Sets { sets } => Ok(sets),
        SetsDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_set_from_body(body: &str) -> Result<SetDetail, Error> {
    use model::set::SetDto;
    match serde_json::from_str::<SetDto>(&body).context(MtgIoErrorKind::SetBodyParseError)? {
        SetDto::Set { set } => Ok(set),
        SetDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_formats_from_body(body: &str) -> Result<Vec<String>, Error> {
    use model::format::FormatDto;
    match serde_json::from_str::<FormatDto>(&body).context(MtgIoErrorKind::FormatBodyParseError)? {
        FormatDto::Formats { formats } => Ok(formats),
        FormatDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_types_from_body(body: &str) -> Result<Vec<String>, Error> {
    use model::types::TypesDto;
    match serde_json::from_str::<TypesDto>(&body).context(MtgIoErrorKind::TypeBodyParseError)? {
        TypesDto::Types { types } => Ok(types),
        TypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_subtypes_from_body(body: &str) -> Result<Vec<String>, Error> {
    use model::types::SubtypesDto;
    match serde_json::from_str::<SubtypesDto>(&body).context(MtgIoErrorKind::TypeBodyParseError)? {
        SubtypesDto::Subtypes { subtypes } => Ok(subtypes),
        SubtypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}

pub(crate) fn retrieve_supertypes_from_body(body: &str) -> Result<Vec<String>, Error> {
    use model::types::SupertypesDto;
    match serde_json::from_str::<SupertypesDto>(&body).context(MtgIoErrorKind::TypeBodyParseError)?
    {
        SupertypesDto::Supertypes { supertypes } => Ok(supertypes),
        SupertypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgIoErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            })?,
            None => Err(MtgIoErrorKind::ApiError { cause: error })?,
        },
    }
}