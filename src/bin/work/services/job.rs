use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        job::{Job},
    }
};
use actix_web::{web, http::StatusCode};

pub fn get_jobs(j: Job,pool: &web::Data<Pool>) -> Result<Vec<Job>, ServiceError> {
    match Job::get_jobs(j, &pool.get().unwrap()) {
        Ok(js) => Ok(js),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

