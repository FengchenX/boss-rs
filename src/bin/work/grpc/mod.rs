
mod greeter;
mod job;

use boss::grpc::*;
use greeter::*;
use job::*;
use tonic::{transport::Server};
use boss::db::*;


pub struct Svr{
    pool: Pool,
}


impl Svr {
    pub fn new(pool: Pool)->Self{
        Svr{pool}
    }
    pub async fn register(&self)-> Result<(), Box<dyn std::error::Error>>{
        let addr = "[::1]:50051".parse()?;
        let greeter = MyGreeter::default();
        let mut job = MyJob{
            pool:self.pool.clone()
        };
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .add_service(JobServer::new(job))
            .serve(addr)
            .await?;

        Ok(())
    }
}