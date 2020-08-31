use rocket_contrib::json::Json;
use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpErrorMessage {
    pub code: u16,
    pub message: &'static str,
}

#[catch(404)]
pub fn not_found() -> Json<HttpErrorMessage> {
    Json(HttpErrorMessage{
        code: Status::NotFound.code,
        message: Status::NotFound.reason,
    })
}

#[catch(500)]
pub fn internal_server_error() -> Json<HttpErrorMessage> {
    Json(HttpErrorMessage{
        code: Status::InternalServerError.code,
        message: Status::InternalServerError.reason,
    })
}
