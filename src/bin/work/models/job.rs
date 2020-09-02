
// use crate::{
//     config::db::{PgConnection, MysqlConnection},
//     constants,
//     schema::job::{self, dsl::*}
// };
// use bcrypt::{hash, verify, DEFAULT_COST};
// use diesel::prelude::*;
// use uuid::Uuid;
//
//
// #[derive(Identifiable, Queryable, Serialize, Deserialize)]
// pub struct Job{
//     pub id: i32,
//     pub creator: i64,
//     pub create_at: i64,
//     pub update_at: i64,
//     pub delete_at: i64,
//     pub progress: f64,
//     pub status: String,
//     pub file: String
// }
//
// impl Job{
//     // pub fn get_jobs(j: Job, conn: &PgConnection) -> Option<Job>{
//     //     if let Ok(js) = job
//     //         .filter(status.eq(&j.status))
//     //         .get_result::<Job>(conn)
//     //     {
//     //         if js.is_empty(){
//     //
//     //         }
//     //     }
//     //     None
//     // }
// }