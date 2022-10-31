use easy_modbus::codec::RtuClientCodec;
use easy_modbus::Frame;
use futures::{SinkExt, StreamExt};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tty_path = "COM4";
    let rate = 9600;
    let slave = 0x01;

    let serial_builder = tokio_serial::new(tty_path, rate);
    let port = SerialStream::open(&serial_builder).unwrap();

    let mut transport = Framed::new(port, RtuClientCodec);

    let frame = Frame::rtu();
    let request = frame.read_multiple_holding_registers_request(slave, 0x00, 0x02);
    println!("{}", request);
    transport.send(request).await?;

    while let Some(response) = transport.next().await {
        match response {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("Hello, world!");
    Ok(())
}
