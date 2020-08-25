
// pub use crate::prelude::*;
// pub use diesel::query_dsl::methods;
//
//
// fn establish_connection(database_url: &str) -> MysqlConnection {
//     // dotenv().ok();
//     // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     MysqlConnection::establish(database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
//
// pub struct DB {
//     pub conn: MysqlConnection,
// }
//
// impl DB {
//     pub fn new(addr: &str) ->Self {
//         let mut ret = DB{
//             conn:establish_connection(addr),
//         };
//         ret
//     }
// }
//
// pub struct BossDB {
//     pub db: DB,
// }
//
// impl BossDB {
//     pub fn new() -> Self {
//         dotenv().ok();
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         // let addr = "mysql://geek:123456@tcp(10.10.116.174:3306)/boss?parseTime=true";
//         BossDB{
//             db: DB::new(&database_url),
//         }
//     }
// }

use diesel::{r2d2::{self, ConnectionManager}};
use diesel::{PgConnection, MysqlConnection};

embed_migrations!();

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub trait PgDB{
    fn migrate_and_config_db(&self, url: &str) -> Pool{
        info!("Migrating and configurating database...");
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
        embedded_migrations::run(&pool.get().expect("Failed to migrate."));

        pool
    }
}


pub type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub trait MysqlDB{
    fn migrate_and_config_db(&self, url: &str) -> MysqlPool{
        info!("Migrating and configurating database...");
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
        embedded_migrations::run(&pool.get().expect("Failed to migrate."));

        pool
    }
}




#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_boss_db(){
    }
}





