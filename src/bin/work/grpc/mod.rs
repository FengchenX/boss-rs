
mod greeter;
use boss::grpc::*;
use greeter::*;
use tonic::{transport::Server};


pub struct Svr;


impl Svr {
    pub fn new()->Self{
        Svr
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