use crate::{
    config::db::{PgConnection,MysqlConnection},
    models::user::User,
    schema::login_history::{self, dsl::*},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

impl LoginHistory {
    pub fn create(un: &str, conn: &PgConnection) -> Option<LoginHistoryInsertableDTO> {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertableDTO, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(conn)
    }
}