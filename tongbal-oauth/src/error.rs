use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
}

enum Kind {
    Setup,
    Build,
    Request,
    Api(Api),
    Parse,
    Csrf,
}

struct Api {
    code: u16,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    Setup,
    Build,
    Request,
    Api,
    Parse,
    Csrf,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Setup => "setup",
            Self::Build => "build",
            Self::Request => "request",
            Self::Api => "api",
            Self::Parse => "parse",
            Self::Csrf => "csrf",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl Error {
    pub fn is_timeout(&self) -> bool {
        self.inner
            .source
            .as_ref()
            .and_then(|e| e.downcast_ref::<reqwest::Error>())
            .is_some_and(reqwest::Error::is_timeout)
    }

    pub fn kind(&self) -> ErrorKind {
        match self.inner.kind {
            Kind::Setup => ErrorKind::Setup,
            Kind::Build => ErrorKind::Build,
            Kind::Request => ErrorKind::Request,
            Kind::Api(_) => ErrorKind::Api,
            Kind::Parse => ErrorKind::Parse,
            Kind::Csrf => ErrorKind::Csrf,
        }
    }

    pub fn status(&self) -> Option<u16> {
        self.api().map(|api| api.code)
    }

    pub fn message(&self) -> Option<&str> {
        self.api().map(|api| api.message.as_ref())
    }

    fn api(&self) -> Option<&Api> {
        match &self.inner.kind {
            Kind::Api(api) => Some(api),
            _ => None,
        }
    }

    fn new(kind: Kind, source: Option<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner { kind, source }),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut builder = f.debug_struct("tongbal_oauth::Error");
        builder.field("kind", &self.kind());

        if let Kind::Api(api) = &self.inner.kind {
            builder.field("code", &api.code);
            builder.field("message", &api.message);
        }

        if let Some(source) = &self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner.kind {
            Kind::Setup => {
                f.write_str("HTTP client has already been initialized and cannot be reconfigured")?
            }
            Kind::Build => f.write_str("failed to build request")?,
            Kind::Request => f.write_str("failed to send request")?,
            Kind::Api(api) => {
                write!(f, "unexpected HTTP status {}", api.code)?;

                if !api.message.is_empty() {
                    write!(f, " {:?}", api.message)?;
                }
            }
            Kind::Parse => f.write_str("failed to deserialize response")?,
            Kind::Csrf => f.write_str("failed to verify CSRF state")?,
        }

        if let (Kind::Parse, Some(source)) = (&self.inner.kind, &self.inner.source) {
            write!(f, ": {source}")?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if matches!(self.inner.kind, Kind::Parse) {
            // rendered by Display
            return None;
        }

        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

pub(crate) fn setup() -> Error {
    Error::new(Kind::Setup, None)
}

pub(crate) fn request(source: impl Into<BoxError>) -> Error {
    match source.into().downcast::<reqwest::Error>() {
        Ok(source) => {
            let kind = if source.is_builder() {
                Kind::Build
            } else {
                Kind::Request
            };

            Error::new(kind, Some(source.without_url().into()))
        }
        Err(source) => Error::new(Kind::Request, Some(source)),
    }
}

pub(crate) fn api(code: u16, message: String) -> Error {
    Error::new(Kind::Api(Api { code, message }), None)
}

pub(crate) fn parse(source: impl Into<BoxError>) -> Error {
    Error::new(Kind::Parse, Some(source.into()))
}

pub(crate) fn csrf(source: impl Into<BoxError>) -> Error {
    Error::new(Kind::Csrf, Some(source.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_sync_static() {
        fn assert<T: Send + Sync + 'static>() {}
        assert::<Error>();
    }
}
