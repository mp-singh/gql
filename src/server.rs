// use payments::bitcoin_server::Bitcoin;

// pub mod payments {
//     tonic::include_proto!("payments");
// }

// pub struct BitcoinService {}

// #[tonic::async_trait]
// impl Bitcoin for BitcoinService {
//     async fn send_payment(
//         &self,
//         request: Request<BtcPaymentReuest>,
//     ) -> Result<Response<BtcPaymentResponse>, Status> {
//         let req = request.into_inner();
//         let reply = BtcPaymentResponse {
//             success: true,
//             message: format!("Payment of {} BTC sent to {}", req.amount, req.to_address).into(),
//         };
//         Ok(Response::new(reply))
//     }
// }

// fn main() {}

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, ResponseAgain};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
    async fn say_hello_again(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<ResponseAgain>, Status> {
        let reply = hello_world::ResponseAgain {
            message: format!("Hello again {}!", request.into_inner().name),
            response_time: 0,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world server running...");
    let addr = "[::1]:8081".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
