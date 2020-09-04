

use tonic::{transport::Server, Request, Response, Status, Code};

pub use job::job_server::{Job as JobI, JobServer};
pub use job::{JobReply, JobRequest};
use boss::db::Pool;
use crate::models::job::*;
use crate::error::ServiceError;
use actix_web::http::StatusCode;

pub mod job {
    tonic::include_proto!("job");
}

pub struct MyJob {
    pub pool: Pool,
}

#[tonic::async_trait]
impl JobI for MyJob {
    async fn get_jobs(
        &self,
        request: Request<JobRequest>,
    ) -> Result<Response<JobReply>, Status> {
        println!("Got a request: {:?}", request);

        let mut j = Job{
            id: 0,
            creator: None,
            create_at: None,
            update_at: None,
            delete_at: None,
            progress: None,
            status: Some(request.into_inner().status),
            file: None
        };

        match Job::get_jobs(j, &self.pool.get().unwrap()) {
            Ok(js) => {
                let mut reply = job::JobReply {
                    list: vec![]
                };
                for elem in js{
                    let Job{ id, creator, create_at, update_at, delete_at, progress, status, file }=elem;
                    let j=job::JobM{ id, creator: creator.unwrap(), create_at: create_at.unwrap(), update_at: update_at.unwrap(), delete_at: delete_at.unwrap(), progress: progress.unwrap(), status: status.unwrap(), file: file.unwrap() };
                    reply.list.push(j);
                }

                Ok(Response::new(reply))
            },
            Err(_) => Err(Status::new(Code::Ok,"find no"))
        }
    }
}
