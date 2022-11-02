use futures::{SinkExt, StreamExt};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;

use easy_modbus::{Frame, Response};
use easy_modbus::codec::RtuClientCodec;

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
    println!("Request:\t{}", request);

    transport.send(request).await?;
    while let Some(response) = transport.next().await {
        match response {
            Ok(response) => {
                println!("Response:\t{}", response);
                match response {
                    Response::ReadMultipleHoldingRegisters(_, res) => {
                        let a = res.get_values();
                        let h = ((a[0] as u16 * 256) + a[1] as u16) as f64 / 10.0;
                        let t = ((a[2] as u16 * 256) + a[3] as u16) as f64 / 10.0;
                        println!("h {} t {}", h, t);
                        return Ok(())
                    }
                    _ => {
                        println!("unknown")
                    }
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
