
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
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
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate jsonwebtoken;
extern crate uuid;
extern crate bcrypt;

pub mod prelude;
pub mod db;
pub mod schema;
pub mod grpc;
