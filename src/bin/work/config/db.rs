use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}, MysqlConnection};

use boss::db::*;

embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub struct MyDB;

impl DB<Connection> for MyDB {

}

pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Migrating and configurating database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}

pub type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn migrate_and_config_msdb(url: &str) -> MysqlPool {
    info!("Migrating and configurating database...");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}