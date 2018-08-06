use failure::Backtrace;
use failure::Context;
use failure::Fail;
use std::fmt;
use std::fmt::Display;

/// Errors encountered by the Client
#[derive(Debug)]
pub struct MtgApiError {
    inner: Context<MtgApiErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum MtgApiErrorKind {
    #[fail(display = "The Client has been dropped and is no longer available")]
    ClientDropped,
    #[fail(display = "Error calling the API Endpoint")]
    HttpError,
    #[fail(display = "Failed to read the response body")]
    BodyReadError,
    #[fail(display = "Could not parse the response of the card struct")]
    CardBodyParseError,
    #[fail(display = "Requested card not found")]
    CardNotFound,
    #[fail(display = "Could not parse the response of the set struct")]
    SetBodyParseError,
    #[fail(display = "Requested set not found")]
    SetNotFound,
    #[fail(display = "Could not parse the response of the types struct")]
    TypeBodyParseError,
    #[fail(display = "Could not parse the response of the subtypes struct")]
    SubtypeBodyParseError,
    #[fail(display = "Could not parse the response of the supertypes struct")]
    SupertypeBodyParseError,
    #[fail(display = "Could not parse the response of the formats struct")]
    FormatBodyParseError,
    #[fail(display = "Error: {}", cause)]
    ApiError { cause: String },
}

impl Fail for MtgApiError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MtgApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl MtgApiError {
    pub fn kind(&self) -> MtgApiErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<MtgApiErrorKind> for MtgApiError {
    fn from(kind: MtgApiErrorKind) -> MtgApiError {
        MtgApiError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MtgApiErrorKind>> for MtgApiError {
    fn from(inner: Context<MtgApiErrorKind>) -> MtgApiError {
        MtgApiError { inner }
    }
}
