use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error(anyhow::Error);

pub type Result<T> = anyhow::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn msg<M>(msg: M) -> Self
    where
        M: Display + std::fmt::Debug + Send + Sync + 'static,
    {
        anyhow::Error::msg(msg).into()
    }
}
