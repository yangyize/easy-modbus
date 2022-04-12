use super::{ClientCodec, ServerCodec};
use crate::frame::request::*;
use crate::frame::response::*;
use crate::frame::{
    request::{ReadCoilsRequest, Request},
    response::{ReadCoilsResponse, Response},
    Function, Head, Mbap,
};
use bytes::{Buf, Bytes, BytesMut};
use std::io::{Error, ErrorKind::InvalidData, Result};
use tokio_util::codec::Decoder;

impl Decoder for ClientCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Response>> {
        if src.len() < 8 {
            return Ok(None);
        }
        let head = Head::from(src.copy_to_bytes(8));
        let len = head.mbap.length as usize - 2;
        let response = match head.function {
            Function::ReadCoils => {
                Response::ReadCoils(head, ReadCoilsResponse::from(src.copy_to_bytes(len)))
            }
            Function::ReadDiscreteInputs => Response::ReadDiscreteInputs(
                head,
                ReadDiscreteInputsResponse::from(src.copy_to_bytes(len)),
            ),
            Function::ReadMultipleHoldingRegisters => Response::ReadMultipleHoldingRegisters(
                head,
                ReadMultipleHoldingRegistersResponse::from(src.copy_to_bytes(len)),
            ),
            Function::ReadInputRegisters => Response::ReadInputRegisters(
                head,
                ReadInputRegistersResponse::from(src.copy_to_bytes(len)),
            ),
            Function::WriteSingleCoil => Response::WriteSingleCoil(
                head,
                WriteSingleCoilResponse::from(src.copy_to_bytes(len)),
            ),
            Function::WriteSingleHoldingRegister => Response::WriteSingleHoldingRegister(
                head,
                WriteSingleHoldingRegisterResponse::from(src.copy_to_bytes(len)),
            ),
            Function::WriteMultipleCoils => Response::WriteMultipleCoils(
                head,
                WriteMultipleCoilsResponse::from(src.copy_to_bytes(len)),
            ),
            Function::WriteMultipleHoldingRegisters => Response::WriteMultipleHoldingRegisters(
                head,
                WriteMultipleHoldingRegistersResponse::from(src.copy_to_bytes(len)),
            ),
        };
        Ok(Some(response))
    }
}

impl Decoder for ServerCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Request>> {
        if src.len() < 8 {
            return Ok(None);
        }
        let head = Head::from(src.copy_to_bytes(8));
        let len = head.mbap.length as usize - 2;
        let request = match head.function {
            Function::ReadCoils => {
                Request::ReadCoils(head, ReadCoilsRequest::from(src.copy_to_bytes(len)))
            }
            Function::ReadDiscreteInputs => Request::ReadDiscreteInputs(
                head,
                ReadDiscreteInputsRequest::from(src.copy_to_bytes(len)),
            ),
            Function::ReadMultipleHoldingRegisters => Request::ReadMultipleHoldingRegisters(
                head,
                ReadMultipleHoldingRegistersRequest::from(src.copy_to_bytes(len)),
            ),
            Function::ReadInputRegisters => Request::ReadInputRegisters(
                head,
                ReadInputRegistersRequest::from(src.copy_to_bytes(len)),
            ),
            Function::WriteSingleCoil => {
                Request::WriteSingleCoil(head, WriteSingleCoilRequest::from(src.copy_to_bytes(len)))
            }
            Function::WriteSingleHoldingRegister => Request::WriteSingleHoldingRegister(
                head,
                WriteSingleHoldingRegisterRequest::from(src.copy_to_bytes(len)),
            ),
            Function::WriteMultipleCoils => Request::WriteMultipleCoils(
                head,
                WriteMultipleCoilsRequest::from(src.copy_to_bytes(len)),
            ),
            Function::WriteMultipleHoldingRegisters => Request::WriteMultipleHoldingRegisters(
                head,
                WriteMultipleHoldingRegistersRequest::from(src.copy_to_bytes(len)),
            ),
        };
        Ok(Some(request))
    }
}

impl From<Bytes> for ReadCoilsRequest {
    fn from(mut buf: Bytes) -> Self {
        ReadCoilsRequest {
            first_address: buf.get_u16(),
            coils_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for ReadDiscreteInputsRequest {
    fn from(mut buf: Bytes) -> Self {
        ReadDiscreteInputsRequest {
            first_address: buf.get_u16(),
            discrete_inputs_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for ReadMultipleHoldingRegistersRequest {
    fn from(mut buf: Bytes) -> Self {
        ReadMultipleHoldingRegistersRequest {
            first_address: buf.get_u16(),
            registers_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for ReadInputRegistersRequest {
    fn from(mut buf: Bytes) -> Self {
        ReadInputRegistersRequest {
            first_address: buf.get_u16(),
            registers_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for WriteSingleCoilRequest {
    fn from(mut buf: Bytes) -> Self {
        WriteSingleCoilRequest {
            coil_address: buf.get_u16(),
            value: buf.get_u16(),
        }
    }
}

impl From<Bytes> for WriteSingleHoldingRegisterRequest {
    fn from(mut buf: Bytes) -> Self {
        WriteSingleHoldingRegisterRequest {
            register_address: buf.get_u16(),
            value: buf.get_u16(),
        }
    }
}

impl From<Bytes> for WriteMultipleCoilsRequest {
    fn from(mut buf: Bytes) -> Self {
        WriteMultipleCoilsRequest {
            first_address: buf.get_u16(),
            coils_number: buf.get_u16(),
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for WriteMultipleHoldingRegistersRequest {
    fn from(mut buf: Bytes) -> Self {
        WriteMultipleHoldingRegistersRequest {
            first_address: buf.get_u16(),
            registers_number: buf.get_u16(),
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for ReadCoilsResponse {
    fn from(mut buf: Bytes) -> Self {
        ReadCoilsResponse {
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for ReadDiscreteInputsResponse {
    fn from(mut buf: Bytes) -> Self {
        ReadDiscreteInputsResponse {
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for ReadMultipleHoldingRegistersResponse {
    fn from(mut buf: Bytes) -> Self {
        ReadMultipleHoldingRegistersResponse {
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for ReadInputRegistersResponse {
    fn from(mut buf: Bytes) -> Self {
        ReadInputRegistersResponse {
            bytes_number: buf.get_u8(),
            values: buf.to_vec(),
        }
    }
}

impl From<Bytes> for WriteSingleCoilResponse {
    fn from(mut buf: Bytes) -> Self {
        WriteSingleCoilResponse {
            coil_address: buf.get_u16(),
            value: buf.get_u16(),
        }
    }
}

impl From<Bytes> for WriteSingleHoldingRegisterResponse {
    fn from(mut buf: Bytes) -> Self {
        WriteSingleHoldingRegisterResponse {
            register_address: buf.get_u16(),
            value: buf.get_u16(),
        }
    }
}

impl From<Bytes> for WriteMultipleCoilsResponse {
    fn from(mut buf: Bytes) -> Self {
        WriteMultipleCoilsResponse {
            first_address: buf.get_u16(),
            coils_number: buf.get_u16(),
        }
    }
}
#[test]
fn read_coils_response_test() {
    let mut codec = ClientCodec::default();
    let v: Vec<u8> = vec![
        0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x01, 0x02, 0x00, 0x01,
    ];
    let mut buf = BytesMut::from(&v[..]);
    let response_l = codec.decode(&mut buf).unwrap().unwrap();
    let response_r = Response::read_coils(0x01, 0x01, vec![0x00, 0x01]);
    assert_eq!(response_l, response_r);
}

impl From<Bytes> for WriteMultipleHoldingRegistersResponse {
    fn from(mut buf: Bytes) -> Self {
        WriteMultipleHoldingRegistersResponse {
            first_address: buf.get_u16(),
            registers_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for Head {
    fn from(mut buf: Bytes) -> Self {
        let mbap = Mbap::from(buf.slice(0..7));
        buf.advance(7);
        let function = Function::try_from(buf.get_u8()).unwrap();
        Head { mbap, function }
    }
}

impl TryFrom<u8> for Function {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self> {
        let func = match value {
            0x01 => Function::ReadCoils,
            0x02 => Function::ReadDiscreteInputs,
            0x03 => Function::ReadMultipleHoldingRegisters,
            0x04 => Function::ReadInputRegisters,
            0x05 => Function::WriteSingleCoil,
            0x06 => Function::WriteSingleHoldingRegister,
            0x0F => Function::WriteMultipleCoils,
            0x10 => Function::WriteMultipleHoldingRegisters,
            _ => {
                return Err(Error::new(
                    InvalidData,
                    format!("Invalid function code: 0x{:0>2X}", value),
                ));
            }
        };
        Ok(func)
    }
}

impl From<Bytes> for Mbap {
    fn from(mut buf: Bytes) -> Self {
        Mbap {
            tid: buf.get_u16(),
            pid: buf.get_u16(),
            length: buf.get_u16(),
            uid: buf.get_u8(),
        }
    }
}

#[cfg(test)]
mod client_decoder_test {
    use crate::{ClientCodec, Response};
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    #[test]
    fn read_coils_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x01, 0x02, 0x00, 0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::read_coils(0x01, 0x01, vec![0x00, 0x01]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_discrete_inputs_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x01, 0x02, 0x03, 0xAC, 0xDB, 0xFB, 0x0D,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::read_discrete(0x01, 0x01, 28, vec![0xAC, 0xDB, 0xFB, 0x0D]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_multiple_holding_registers_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x03, 0x06, 0xAE, 0x41, 0x56, 0x52, 0x43,
            0x40,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r =
            Response::read_holding_register(0x01, 0x01, vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_input_registers_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x04, 0x02, 0x10, 0x2F,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::read_input_register(0x01, 0x01, vec![0x10, 0x2F]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_coil_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0xBF, 0x00, 0x00,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::write_single_coil(0x01, 0x01, 0x00BF, 0x0000);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_holding_register_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x04, 0xAB, 0xCD,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::write_single_holding_register(0x01, 0x01, 0x0004, 0xABCD);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_coils_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::write_multiple_coils(0x01, 0x01, 0x001B, 0x0009);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_holding_registers_response_test() {
        let mut codec = ClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x10, 0x00, 0x12, 0x00, 0x02,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let response_r = Response::write_multiple_holding_registers(0x01, 0x01, 0x0012, 0x0002);
        assert_eq!(response_l, response_r);
    }
}

#[cfg(test)]
mod server_decoder_test {
    use crate::{Request, ServerCodec};
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;
    #[test]
    fn read_coils_request_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x01, 0x00, 0x02, 0x00, 0x08,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::read_coils(0x01, 0x01, 0x02, 0x08);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_discrete_inputs_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x00, 0x7A, 0x00, 0x1C,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::read_discrete(0x01, 0x01, 0x007A, 0x001C);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_multiple_holding_registers_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0x00, 0x6F, 0x00, 0x03,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::read_multiple_holding_registers(0x01, 0x01, 0x006F, 0x0003);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_input_registers_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x04, 0x00, 0x0A, 0x00, 0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::read_input_registers(0x01, 0x01, 0x000A, 0x0001);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_coil_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0xBF, 0x00, 0x00,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::write_single_coil(0x01, 0x01, 0x00BF, 0x0000);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_holding_register_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x04, 0xAB, 0xCD,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::write_single_holding_register(0x01, 0x01, 0x0004, 0xABCD);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_coils_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D,
            0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::write_multiple_coils(0x01, 0x01, 0x001B, 0x0009, vec![0x4D, 0x01]);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_holding_registers_test() {
        let mut codec = ServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x0B, 0x01, 0x10, 0x00, 0x12, 0x00, 0x02, 0x04, 0x0B,
            0x0A, 0xC1, 0x02,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let request_r = Request::write_multiple_holding_registers(
            0x01,
            0x01,
            0x0012,
            vec![0x0B, 0x0A, 0xC1, 0x02],
        );
        assert_eq!(request_l, request_r);
    }
}
