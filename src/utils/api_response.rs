use crate::services::db::DatabaseResponse;
use actix_web::{body::BoxBody, http::StatusCode, web, HttpRequest, HttpResponse, Responder};

pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    pub response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
    // pub fn new_from_macro(response: DatabaseResponse) -> Self {
    //     ApiResponse {
    //         status_code: response.error_code,
    //         body: response.message,
    //         response_code: StatusCode::from_u16(response.error_code).unwrap(),
    //     }
    // }
    pub fn new_from_macro(response: String) -> Self {
        println!("{response}");
        ApiResponse {
            status_code: 500,
            body: response,
            response_code: StatusCode::from_u16(500).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}
