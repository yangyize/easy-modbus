use easy_modbus::{Response, ServerCodec};
use futures::SinkExt;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8081".to_string();
    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to  process connection; error = {}", e);
            }
        });
    }
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, ServerCodec);

    while let Some(request) = transport.next().await {
        match request {
            Ok(request) => {
                println!("load request --- {:?}", request);
                let response = Response::read_coils(0x01, 0x01, vec![0x00, 0x01]);
                println!("send response --- {:?}", response);
                transport.send(response).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
