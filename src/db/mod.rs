
use diesel::{r2d2::{self, ConnectionManager}};
pub use diesel::{PgConnection, MysqlConnection};

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





