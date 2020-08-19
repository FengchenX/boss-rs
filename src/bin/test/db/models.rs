use diesel::prelude::*;

#[derive(Queryable)]
pub struct Job {
    pub id: i32,
    pub creator: i64,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub progress: f64,
    pub status: String,
    pub file: String,
}