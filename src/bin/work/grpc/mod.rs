
mod greeter;
use boss::grpc::*;
use greeter::*;
use tonic::{transport::Server};


pub struct Svr{pub x:i32}


impl Svr {
    pub fn new()->Self{
        Svr{x:10}
    }
    pub async fn register(&self)-> Result<(), Box<dyn std::error::Error>>{
        println!("register *******");
        let addr = "[::1]:50051".parse()?;
        let greeter = MyGreeter::default();
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await?;

        println!("register 1111");
        Ok(())
    }

    pub fn test(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("****************hello");

        let addr = "[::1]:50051".parse()?;
        let greeter = MyGreeter::default();
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr);

        println!("test 1111");
        Ok(())
    }
}