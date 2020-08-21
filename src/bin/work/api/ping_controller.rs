use actix_web::HttpResponse;
use actix_web::{web};
use crate::MyData;

#[get("/ping")]
pub fn ping(m: web::Data<MyData>) -> HttpResponse {
    println!("{}",m.counter.get());
    HttpResponse::Ok()
        .body("pong!".to_string())
}
