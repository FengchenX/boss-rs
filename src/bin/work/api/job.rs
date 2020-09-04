use crate::{
    config::db::*,
    constants,
    models::{
        response::ResponseBody,
        job::Job,
    },
    services::job,
};
use actix_web::{web, HttpResponse, Result};


// GET /api/jobs
pub async fn get_jobs(from: web::Query<Job>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match job::get_jobs(from.into_inner(), &pool) {
        Ok(js) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, js))),
        Err(err) => Ok(err.response()),
    }
}

