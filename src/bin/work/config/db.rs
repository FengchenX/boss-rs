pub use boss::db::*;

pub struct BossDB;

impl PgDB for BossDB{}

impl BossDB{
    pub fn new()->Self{
        BossDB
    }
}

