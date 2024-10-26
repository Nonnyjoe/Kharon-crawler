use crate::utils::api_response::ApiResponse;
use actix_web::get;

#[get("/health")]
pub async fn health_check() -> ApiResponse {
    ApiResponse::new(200, "Server is healthy".to_string())
}
