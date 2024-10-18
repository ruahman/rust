use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

pub type Result<T> = core::result::Result<T, Error>;

// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), Error> {
//         write!(fmt, "{self:?}")
//     }
// }

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("INTO_RES: {self:?}");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
