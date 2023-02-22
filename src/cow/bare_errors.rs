use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct BareError {
    pub code: String,
    pub id: String,
    pub message: String,
}

impl Responder for BareError {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        return match self.code.as_str() {
            "UNKNOWN" | "HOST_NOT_FOUND" | "CONNECTION_RESET" | "CONNECTION_REFUSED"
            | "CONNECTION_TIMEOUT" => HttpResponse::InternalServerError(),
            "MISSING_BARE_HEADER"
            | "INVALID_BARE_HEADER"
            | "UNKNOWN_BARE_HEADER"
            | "INVALID_HEADER" => HttpResponse::BadRequest(),
            "FORBIDDEN_BARE_HEADER" => HttpResponse::Forbidden(),
            _ => unreachable!(),
        }
        .content_type(ContentType::json())
        .body(body);
    }
}
