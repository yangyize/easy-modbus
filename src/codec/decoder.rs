use std::io::{Error, ErrorKind::InvalidData, Result};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::Decoder;

use crate::codec::{RtuClientCodec, RtuServerCodec};
use crate::util::crc;
use crate::frame::request::*;
use crate::frame::response::*;
use crate::frame::{
    request::{ReadCoilsRequest, Request},
    response::{ReadCoilsResponse, Response},
    Exception, Function, Head, Version,
};

use super::{TcpClientCodec, TcpServerCodec};

impl Decoder for RtuClientCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Response>> {
        if src.len() < 2 {
            return Ok(None);
        }

        let mut data_bytes = BytesMut::new();

        let head_bytes = src.copy_to_bytes(2);
        data_bytes.put_slice(&(head_bytes.to_vec()));
        let mut head = Head::rtu_try_from(head_bytes)?;

        let len: usize = {
            if head.is_exception {
                1
            } else {
                match head.function {
                    Function::ReadCoils
                    | Function::ReadDiscreteInputs
                    | Function::ReadMultipleHoldingRegisters
                    | Function::ReadInputRegisters => {
                        src.get(0).map_or(0, |&bytes_num| bytes_num as usize + 1)
                    }
                    Function::WriteSingleCoil
                    | Function::WriteSingleHoldingRegister
                    | Function::WriteMultipleCoils
                    | Function::WriteMultipleHoldingRegisters => 4,
                }
            }
        };

        if src.len() < len + 2 {
            return Ok(None);
        }

        head.body_length(len as u16);

        let body_bytes = src.copy_to_bytes(len);
        data_bytes.put_slice(&(body_bytes.to_vec()));
        let response = get_response(body_bytes, head);

        let crc = src.get_u16();
        if crc::check(&(data_bytes.to_vec()), crc) {
            return Ok(Some(response));
        }
        return Err(Error::new(
            InvalidData,
            format!("Invalid crc code: 0x{:0>2X}", crc),
        ));
    }
}

impl Decoder for RtuServerCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Request>> {
        if src.len() < 2 {
            return Ok(None);
        }

        let mut data_bytes = BytesMut::new();
        let head_bytes = src.copy_to_bytes(2);
        data_bytes.put_slice(&(head_bytes.to_vec()));
        let mut head = Head::rtu_try_from(head_bytes)?;

        let len: usize = {
            match head.function {
                Function::ReadCoils
                | Function::ReadDiscreteInputs
                | Function::ReadMultipleHoldingRegisters
                | Function::ReadInputRegisters
                | Function::WriteSingleCoil
                | Function::WriteSingleHoldingRegister => 4,
                Function::WriteMultipleCoils | Function::WriteMultipleHoldingRegisters => {
                    src.get(4).map_or(0, |&bytes_num| bytes_num as usize + 5)
                }
            }
        };
        if src.len() < len + 2 {
            return Ok(None);
        }

        head.body_length(len as u16);
        let body_bytes = src.copy_to_bytes(len);
        data_bytes.put_slice(&(body_bytes.to_vec()));
        let request = get_request(body_bytes, head);
        let crc = src.get_u16();
        if crc::check(&(data_bytes.to_vec()), crc) {
            return Ok(Some(request));
        }
        return Err(Error::new(
            InvalidData,
            format!("Invalid crc code: 0x{:0>2X}", crc),
        ));
    }
}

impl Decoder for TcpClientCodec {
    type Item = Response;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Response>> {
        if src.len() < 4 {
            return Ok(None);
        }
        let head = Head::tcp_try_from(src.copy_to_bytes(8))?;
        let len = head.length as usize - 2;
        let response = get_response(src.copy_to_bytes(len), head);
        Ok(Some(response))
    }
}

impl Decoder for TcpServerCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Request>> {
        if src.len() < 8 {
            return Ok(None);
        }
        let head = Head::tcp_try_from(src.copy_to_bytes(8))?;
        let len = head.length as usize - 2;
        let request = get_request(src.copy_to_bytes(len), head);
        Ok(Some(request))
    }
}

fn get_request(src: Bytes, head: Head) -> Request {
    match head.function {
        Function::ReadCoils => Request::ReadCoils(head, ReadCoilsRequest::from(src)),
        Function::ReadDiscreteInputs => {
            Request::ReadDiscreteInputs(head, ReadDiscreteInputsRequest::from(src))
        }
        Function::ReadMultipleHoldingRegisters => Request::ReadMultipleHoldingRegisters(
            head,
            ReadMultipleHoldingRegistersRequest::from(src),
        ),
        Function::ReadInputRegisters => {
            Request::ReadInputRegisters(head, ReadInputRegistersRequest::from(src))
        }
        Function::WriteSingleCoil => {
            Request::WriteSingleCoil(head, WriteSingleCoilRequest::from(src))
        }
        Function::WriteSingleHoldingRegister => {
            Request::WriteSingleHoldingRegister(head, WriteSingleHoldingRegisterRequest::from(src))
        }
        Function::WriteMultipleCoils => {
            Request::WriteMultipleCoils(head, WriteMultipleCoilsRequest::from(src))
        }
        Function::WriteMultipleHoldingRegisters => Request::WriteMultipleHoldingRegisters(
            head,
            WriteMultipleHoldingRegistersRequest::from(src),
        ),
    }
}

fn get_response(src: Bytes, head: Head) -> Response {
    if head.is_exception {
        return Response::Exception(head, ExceptionResponse::from(src));
    }

    match head.function {
        Function::ReadCoils => Response::ReadCoils(head, ReadCoilsResponse::from(src)),
        Function::ReadDiscreteInputs => {
            Response::ReadDiscreteInputs(head, ReadDiscreteInputsResponse::from(src))
        }
        Function::ReadMultipleHoldingRegisters => Response::ReadMultipleHoldingRegisters(
            head,
            ReadMultipleHoldingRegistersResponse::from(src),
        ),
        Function::ReadInputRegisters => {
            Response::ReadInputRegisters(head, ReadInputRegistersResponse::from(src))
        }
        Function::WriteSingleCoil => {
            Response::WriteSingleCoil(head, WriteSingleCoilResponse::from(src))
        }
        Function::WriteSingleHoldingRegister => Response::WriteSingleHoldingRegister(
            head,
            WriteSingleHoldingRegisterResponse::from(src),
        ),
        Function::WriteMultipleCoils => {
            Response::WriteMultipleCoils(head, WriteMultipleCoilsResponse::from(src))
        }
        Function::WriteMultipleHoldingRegisters => Response::WriteMultipleHoldingRegisters(
            head,
            WriteMultipleHoldingRegistersResponse::from(src),
        ),
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

impl From<Bytes> for WriteMultipleHoldingRegistersResponse {
    fn from(mut buf: Bytes) -> Self {
        WriteMultipleHoldingRegistersResponse {
            first_address: buf.get_u16(),
            registers_number: buf.get_u16(),
        }
    }
}

impl From<Bytes> for ExceptionResponse {
    fn from(mut buf: Bytes) -> Self {
        ExceptionResponse {
            exception: Exception::try_from(buf.get_u8()).unwrap(),
        }
    }
}

impl Head {
    fn tcp_try_from(mut buf: Bytes) -> Result<Self> {
        let tid = buf.get_u16();
        let pid = buf.get_u16();
        let length = buf.get_u16();
        let uid = buf.get_u8();
        let (function, is_exception) = get_function(buf.get_u8())?;
        Ok(Head {
            tid,
            pid,
            length,
            uid,
            function,
            version: Version::Tcp,
            is_exception,
        })
    }

    fn rtu_try_from(mut buf: Bytes) -> Result<Self> {
        let uid = buf.get_u8();
        let (function, is_exception) = get_function(buf.get_u8())?;
        Ok(Head {
            tid: 0,
            pid: 0,
            length: 0,
            uid,
            function,
            version: Version::Rtu,
            is_exception,
        })
    }
}

impl TryFrom<u8> for Exception {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match Exception::from_code(value) {
            None => {
                return Err(Error::new(
                    InvalidData,
                    format!("Invalid Exception code: 0x{:0>2X}", value),
                ));
            }
            Some(exception) => Ok(exception),
        }
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
                    Exception::IllegalFunction.as_error_kind(),
                    format!("Invalid function code: 0x{:0>2X}", value),
                ));
            }
        };
        Ok(func)
    }
}

fn get_function(function_code: u8) -> Result<(Function, bool)> {
    let function: Function;
    let mut is_exception = false;
    if function_code <= 0x80 {
        function = Function::try_from(function_code)?;
    } else {
        function = Function::try_from(function_code - 0x80)?;
        is_exception = true;
    }
    Ok((function, is_exception))
}

#[cfg(test)]
mod rtu_client_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    use crate::frame::{Exception, Function};
    use crate::Frame;
    use crate::codec::RtuClientCodec;

    #[test]
    fn read_coils_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x01, 0x04, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.read_coils_response(0x0B, vec![0xCD, 0x6B, 0xB2, 0x7F]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_discrete_inputs_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x02, 0x04, 0xAC, 0xDB, 0xFB, 0x0D, 0x82, 0x7C];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.read_discrete_response(0x0B, vec![0xAC, 0xDB, 0xFB, 0x0D]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_multiple_holding_registers_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![
            0x0B, 0x03, 0x06, 0xAE, 0x41, 0x56, 0x52, 0x43, 0x40, 0xFA, 0xCD,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r =
            frame.read_holding_register_response(0x0B, vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_input_registers_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x04, 0x02, 0x10, 0x2F, 0x6D, 0x2D];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.read_input_register_response(0x0B, vec![0x10, 0x2F]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_coil_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x05, 0x00, 0xBF, 0x00, 0x00, 0xFC, 0x84];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.write_single_coil_response(0x0B, 0x00BF, 0x0000);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_holding_register_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x006, 0x000, 0x004, 0x0AB, 0x0CD, 0x076, 0x004];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.write_single_holding_register_response(0x0B, 0x0004, 0xABCD);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_coils_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0xE5, 0x60];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.write_multiple_coils_response(0x0B, 0x001B, 0x0009);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_holding_registers_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x10, 0x00, 0x12, 0x00, 0x02, 0xE1, 0x67];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r = frame.write_multiple_holding_registers_response(0x0B, 0x0012, 0x0002);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn exception_response_test() {
        let mut codec = RtuClientCodec::default();
        let v: Vec<u8> = vec![0x0A, 0x81, 0x02, 0xB0, 0x53];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let response_r =
            frame.exception_response(0x0A, Function::ReadCoils, Exception::IllegalDataAddress);
        assert_eq!(response_l, response_r);
    }
}

#[cfg(test)]
mod tcp_client_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    use crate::frame::{Exception, Function};
    use crate::{Frame, codec::TcpClientCodec};

    #[test]
    fn read_coils_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x01, 0x02, 0x00, 0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.read_coils_response(0x01, vec![0x00, 0x01]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_discrete_inputs_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x01, 0x02, 0x04, 0xAC, 0xDB, 0xFB, 0x0D,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.read_discrete_response(0x01, vec![0xAC, 0xDB, 0xFB, 0x0D]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_multiple_holding_registers_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x03, 0x06, 0xAE, 0x41, 0x56, 0x52, 0x43,
            0x40,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r =
            frame.read_holding_register_response(0x01, vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn read_input_registers_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x04, 0x02, 0x10, 0x2F,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.read_input_register_response(0x01, vec![0x10, 0x2F]);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_coil_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0xBF, 0x00, 0x00,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.write_single_coil_response(0x01, 0x00BF, 0x0000);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_single_holding_register_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x04, 0xAB, 0xCD,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.write_single_holding_register_response(0x01, 0x0004, 0xABCD);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_coils_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.write_multiple_coils_response(0x01, 0x001B, 0x0009);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn write_multiple_holding_registers_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x10, 0x00, 0x12, 0x00, 0x02,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r = frame.write_multiple_holding_registers_response(0x01, 0x0012, 0x0002);
        assert_eq!(response_l, response_r);
    }

    #[test]
    fn exception_response_test() {
        let mut codec = TcpClientCodec::default();
        let v: Vec<u8> = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x0A, 0x81, 0x02];
        let mut buf = BytesMut::from(&v[..]);
        let response_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let response_r =
            frame.exception_response(0x0A, Function::ReadCoils, Exception::IllegalDataAddress);
        assert_eq!(response_l, response_r);
    }
}

#[cfg(test)]
mod rtu_server_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    use crate::frame::Frame;
    use crate::codec::RtuServerCodec;

    #[test]
    fn read_coils_request_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x01, 0x00, 0x1D, 0x00, 0x1F, 0xED, 0x6E];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();

        let frame = Frame::rtu();
        let request_r = frame.read_coils_request(0x0B, 0x001D, 0x001F);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_discrete_inputs_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x02, 0x00, 0x7A, 0x00, 0x1C, 0x58, 0xB0];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.read_discrete_request(0x0B, 0x007A, 0x001C);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_multiple_holding_registers_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x03, 0x00, 0x6F, 0x00, 0x03, 0x35, 0x7C];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.read_multiple_holding_registers_request(0x0B, 0x006F, 0x0003);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_input_registers_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x04, 0x00, 0x0A, 0x00, 0x01, 0x11, 0x62];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.read_input_registers_request(0x0B, 0x000A, 0x0001);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_coil_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x05, 0x00, 0xBF, 0x00, 0x00, 0xFC, 0x84];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.write_single_coil_request(0x0B, 0x00BF, 0x0000);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_holding_register_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![0x0B, 0x06, 0x00, 0x04, 0xAB, 0xCD, 0x76, 0x04];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.write_single_holding_register_request(0x0B, 0x0004, 0xABCD);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_coils_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![
            0x0B, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D, 0x01, 0x6C, 0xA7,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.write_multiple_coils_request(0x0B, 0x001B, 0x0009, vec![0x4D, 0x01]);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_holding_registers_test() {
        let mut codec = RtuServerCodec::default();
        let v: Vec<u8> = vec![
            0x0B, 0x10, 0x00, 0x12, 0x00, 0x02, 0x04, 0x0B, 0x0A, 0xC1, 0x02, 0xA0, 0xD5,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::rtu();
        let request_r = frame.write_multiple_holding_registers_request(
            0x0B,
            0x0012,
            vec![0x0B, 0x0A, 0xC1, 0x02],
        );
        assert_eq!(request_l, request_r);
    }
}

#[cfg(test)]
mod tcp_server_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    use crate::frame::Frame;
    use crate::codec::TcpServerCodec;

    #[test]
    fn read_coils_request_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x01, 0x00, 0x02, 0x00, 0x08,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();

        let frame = Frame::tcp();
        let request_r = frame.read_coils_request(0x01, 0x02, 0x08);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_discrete_inputs_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x00, 0x7A, 0x00, 0x1C,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.read_discrete_request(0x01, 0x007A, 0x001C);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_multiple_holding_registers_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0x00, 0x6F, 0x00, 0x03,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.read_multiple_holding_registers_request(0x01, 0x006F, 0x0003);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn read_input_registers_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x04, 0x00, 0x0A, 0x00, 0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.read_input_registers_request(0x01, 0x000A, 0x0001);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_coil_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0xBF, 0x00, 0x00,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.write_single_coil_request(0x01, 0x00BF, 0x0000);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_single_holding_register_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x04, 0xAB, 0xCD,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.write_single_holding_register_request(0x01, 0x0004, 0xABCD);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_coils_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D,
            0x01,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.write_multiple_coils_request(0x01, 0x001B, 0x0009, vec![0x4D, 0x01]);
        assert_eq!(request_l, request_r);
    }

    #[test]
    fn write_multiple_holding_registers_test() {
        let mut codec = TcpServerCodec::default();
        let v: Vec<u8> = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x0B, 0x01, 0x10, 0x00, 0x12, 0x00, 0x02, 0x04, 0x0B,
            0x0A, 0xC1, 0x02,
        ];
        let mut buf = BytesMut::from(&v[..]);
        let request_l = codec.decode(&mut buf).unwrap().unwrap();
        let frame = Frame::tcp();
        let request_r = frame.write_multiple_holding_registers_request(
            0x01,
            0x0012,
            vec![0x0B, 0x0A, 0xC1, 0x02],
        );
        assert_eq!(request_l, request_r);
    }
}
