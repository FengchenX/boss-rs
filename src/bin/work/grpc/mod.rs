
mod greeter;
use boss::grpc::*;
use greeter::*;
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
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await?;

        Ok(())
    }
}