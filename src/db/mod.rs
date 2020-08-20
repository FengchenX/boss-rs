
pub use crate::prelude::*;
pub use diesel::query_dsl::methods;


fn establish_connection(database_url: &str) -> MysqlConnection {
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct DB {
    pub conn: MysqlConnection,
}

impl DB {
    pub fn new(addr: &str) ->Self {
        let mut ret = DB{
            conn:establish_connection(addr),
        };
        ret
    }
}

pub struct BossDB {
    pub db: DB,
}

impl BossDB {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let addr = "mysql://geek:123456@tcp(10.10.116.174:3306)/boss?parseTime=true";
        BossDB{
            db: DB::new(&database_url),
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use models::*;
    use crate::schema::posts::dsl::*;

    #[test]
    fn test_boss_db(){
        let a = BossDB::new();
        assert_eq!(a, "");
        // let results = posts
        //     .filter(published.eq(true))
        //     .limit(5)
        //     .load::<Post>(&a.db.conn)
        //     .expect("Error loading posts");
        // println!("Displaying {} posts", results.len());
        // for post in results {
        //     // println!("{}", post.title);
        //     // println!("-----------\n");
        //     // println!("{}", post.body);
        //     assert_eq!(post.title,"fdfdf");
        // }
    }
}





