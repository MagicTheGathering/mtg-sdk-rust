use failure::Backtrace;
use failure::Context;
use failure::Fail;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MtgIoError {
    inner: Context<MtgIoErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum MtgIoErrorKind {
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
}

impl Fail for MtgIoError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MtgIoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl MtgIoError {
    pub fn kind(&self) -> MtgIoErrorKind {
        *self.inner.get_context()
    }
}

impl From<MtgIoErrorKind> for MtgIoError {
    fn from(kind: MtgIoErrorKind) -> MtgIoError {
        MtgIoError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MtgIoErrorKind>> for MtgIoError {
    fn from(inner: Context<MtgIoErrorKind>) -> MtgIoError {
        MtgIoError { inner }
    }
}
