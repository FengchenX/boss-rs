

use tonic::{transport::Server, Request, Response, Status};

pub use job::job_server::{Job, JobServer};
pub use job::{JobReply, JobRequest};
use boss::db::Pool;

pub mod job {
    tonic::include_proto!("job");
}

pub struct MyJob {
    pub pool: Pool,
}

#[tonic::async_trait]
impl Job for MyJob {
    async fn get_jobs(
        &self,
        request: Request<JobRequest>,
    ) -> Result<Response<JobReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = job::JobReply {
            list: vec![]
        };

        Ok(Response::new(reply))
    }
}
