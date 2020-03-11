use crate::api::error::MtgApiErrorKind;
use failure::Error;
use failure::ResultExt;
use crate::model::card::CardDetail;
use crate::model::set::SetDetail;
use reqwest::Client;
use reqwest::Response;
use serde_json;
use std::rc::Weak;

pub(crate) fn send_response(url: &str, client: &Weak<Client>) -> Result<Response, Error> {
    let client = match client.upgrade() {
        Some(client) => Ok(client),
        None => Err(MtgApiErrorKind::ClientDropped),
    }?;
    info!("GET; {}", &url);
    Ok(client.get(url).send().context(MtgApiErrorKind::HttpError)?)
}

pub(crate) fn retrieve_cards_from_body(body: &str) -> Result<Vec<CardDetail>, Error> {
    use crate::model::card::CardsDto;
    match serde_json::from_str::<CardsDto>(&body).context(MtgApiErrorKind::CardBodyParseError)? {
        CardsDto::Cards { cards } => Ok(cards),
        CardsDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_card_from_body(body: &str) -> Result<Box<CardDetail>, Error> {
    use crate::model::card::CardDto;
    match serde_json::from_str::<CardDto>(&body).context(MtgApiErrorKind::CardBodyParseError)? {
        CardDto::Card { card } => Ok(card),
        CardDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_sets_from_body(body: &str) -> Result<Vec<SetDetail>, Error> {
    use crate::model::set::SetsDto;
    match serde_json::from_str::<SetsDto>(&body).context(MtgApiErrorKind::SetBodyParseError)? {
        SetsDto::Sets { sets } => Ok(sets),
        SetsDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_set_from_body(body: &str) -> Result<Box<SetDetail>, Error> {
    use crate::model::set::SetDto;
    match serde_json::from_str::<SetDto>(&body).context(MtgApiErrorKind::SetBodyParseError)? {
        SetDto::Set { set } => Ok(set),
        SetDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_formats_from_body(body: &str) -> Result<Vec<String>, Error> {
    use crate::model::format::FormatDto;
    match serde_json::from_str::<FormatDto>(&body).context(MtgApiErrorKind::FormatBodyParseError)? {
        FormatDto::Formats { formats } => Ok(formats),
        FormatDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_types_from_body(body: &str) -> Result<Vec<String>, Error> {
    use crate::model::types::TypesDto;
    match serde_json::from_str::<TypesDto>(&body).context(MtgApiErrorKind::TypeBodyParseError)? {
        TypesDto::Types { types } => Ok(types),
        TypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_subtypes_from_body(body: &str) -> Result<Vec<String>, Error> {
    use crate::model::types::SubtypesDto;
    match serde_json::from_str::<SubtypesDto>(&body).context(MtgApiErrorKind::TypeBodyParseError)? {
        SubtypesDto::Subtypes { subtypes } => Ok(subtypes),
        SubtypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}

pub(crate) fn retrieve_supertypes_from_body(body: &str) -> Result<Vec<String>, Error> {
    use crate::model::types::SupertypesDto;
    match serde_json::from_str::<SupertypesDto>(&body).context(MtgApiErrorKind::TypeBodyParseError)?
    {
        SupertypesDto::Supertypes { supertypes } => Ok(supertypes),
        SupertypesDto::Error { error, status } => match status {
            Some(status) => Err(MtgApiErrorKind::ApiError {
                cause: format!("{}: {}", status, error),
            }.into()),
            None => Err(MtgApiErrorKind::ApiError { cause: error }.into()),
        },
    }
}