use actix_web::HttpResponse;
use actix_web::{web};
use crate::MyData;

#[get("/ping")]
pub fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("pong!".to_string())
}
