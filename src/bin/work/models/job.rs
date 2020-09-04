
use crate::{
    config::db::{PgConnection, MysqlConnection},
    constants,
    schema::job::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;


#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name="job"]
pub struct Job{
    pub id: i32,
    pub creator: Option<i64>,
    pub create_at: Option<i64>,
    pub update_at: Option<i64>,
    pub delete_at: Option<i64>,
    pub progress: Option<f64>,
    pub status: Option<String>,
    pub file: Option<String>
}

impl Job {
    pub fn get_jobs(j: Job, conn: &PgConnection) -> QueryResult<Vec<Job>>{
        job
            .filter(status.eq(&j.status))
            .load::<Job>(conn)
    }
}