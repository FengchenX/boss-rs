
#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_rt;
extern crate actix_cors;
extern crate env_logger;
extern crate serde;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate jsonwebtoken;
extern crate uuid;
extern crate bcrypt;

extern crate tokio;
use tokio::prelude::*;

mod api;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
use boss::schema;
mod services;
mod utils;
mod grpc;

use actix_web::{http, HttpServer, App};
use actix_service::Service;
use futures::FutureExt;
use std::{io, env, thread};
use std::default::Default;
use actix_cors::Cors;

use std::cell::Cell;
use boss::db::*;

#[derive(Debug, Clone)]
struct MyData {
    counter: Cell<usize>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let boss = config::db::BossDB::new();
    let pool = boss.migrate_and_config_db(&db_url);

    let my = MyData{counter: Cell::new(10)};


    tokio::spawn(async move{
        let svr = grpc::Svr::new();
        svr.register().await;
    });

    HttpServer::new(move || {
        println!("this is thread web");
        App::new()
            .wrap(Cors::new() // allowed_origin return access-control-allow-origin: * by default
                // .allowed_origin("http://127.0.0.1:8080")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
            .data(pool.clone())
            // .data(mspool.clone())
            .data(my.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::authen_middleware::Authentication)
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| res)
            })
            .configure(config::app::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}