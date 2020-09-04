
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use job::job_client::JobClient;
use job::JobRequest;

pub mod job {
    tonic::include_proto!("job");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    let mut jobclient = JobClient::connect("http://[::1]:50051").await?;

    let jobrequest = tonic::Request::new(JobRequest {
       status: String::from("error"),
    });

    let jobresponse = jobclient.get_jobs(jobrequest).await?;

    println!("RESPONSE={:?}", jobresponse);
    Ok(())
}