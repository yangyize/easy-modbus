use super::ServerCodec;
use crate::codec::ClientCodec;
use crate::frame::request::*;
use crate::frame::response::*;
use crate::frame::{
    request::ReadCoilsRequest,
    response::{ReadCoilsResponse, Response},
    Function, Head, Mbap,
};
use bytes::{BufMut, BytesMut};
use std::io::{Error, Result};
use tokio_util::codec::Encoder;

impl Encoder<Request> for ClientCodec {
    type Error = Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<()> {
        match item {
            Request::ReadCoils(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::ReadDiscreteInputs(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::ReadMultipleHoldingRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::ReadInputRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::WriteSingleCoil(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::WriteSingleHoldingRegister(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::WriteMultipleCoils(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Request::WriteMultipleHoldingRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
        };
        Ok(())
    }
}

impl Encoder<Response> for ServerCodec {
    type Error = Error;

    fn encode(&mut self, item: Response, dst: &mut BytesMut) -> Result<()> {
        match item {
            Response::ReadCoils(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::ReadDiscreteInputs(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::ReadMultipleHoldingRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::ReadInputRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::WriteSingleCoil(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::WriteSingleHoldingRegister(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::WriteMultipleCoils(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
            Response::WriteMultipleHoldingRegisters(head, body) => {
                dst.put(BytesMut::from(head));
                dst.put(BytesMut::from(body));
            }
        }
        Ok(())
    }
}

impl From<ReadCoilsRequest> for BytesMut {
    fn from(request: ReadCoilsRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.coils_number);
        buf
    }
}

impl From<ReadDiscreteInputsRequest> for BytesMut {
    fn from(request: ReadDiscreteInputsRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.discrete_inputs_number);
        buf
    }
}

impl From<ReadMultipleHoldingRegistersRequest> for BytesMut {
    fn from(request: ReadMultipleHoldingRegistersRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.registers_number);
        buf
    }
}

impl From<ReadInputRegistersRequest> for BytesMut {
    fn from(request: ReadInputRegistersRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.registers_number);
        buf
    }
}

impl From<WriteSingleCoilRequest> for BytesMut {
    fn from(request: WriteSingleCoilRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.coil_address);
        buf.put_u16(request.value);
        buf
    }
}

impl From<WriteSingleHoldingRegisterRequest> for BytesMut {
    fn from(request: WriteSingleHoldingRegisterRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.register_address);
        buf.put_u16(request.value);
        buf
    }
}

impl From<WriteMultipleCoilsRequest> for BytesMut {
    fn from(request: WriteMultipleCoilsRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.coils_number);
        buf.put_u8(request.bytes_number);
        buf.put_slice(request.values.as_slice());
        buf
    }
}

impl From<WriteMultipleHoldingRegistersRequest> for BytesMut {
    fn from(request: WriteMultipleHoldingRegistersRequest) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(request.first_address);
        buf.put_u16(request.registers_number);
        buf.put_u8(request.bytes_number);
        buf.put_slice(request.values.as_slice());
        buf
    }
}

impl From<ReadCoilsResponse> for BytesMut {
    fn from(response: ReadCoilsResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u8(response.bytes_number);
        buf.put_slice(response.values.as_slice());
        buf
    }
}

impl From<ReadDiscreteInputsResponse> for BytesMut {
    fn from(response: ReadDiscreteInputsResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u8(response.bytes_number);
        buf.put_slice(response.values.as_slice());
        buf
    }
}

impl From<ReadMultipleHoldingRegistersResponse> for BytesMut {
    fn from(response: ReadMultipleHoldingRegistersResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u8(response.bytes_number);
        buf.put_slice(response.values.as_slice());
        buf
    }
}

impl From<ReadInputRegistersResponse> for BytesMut {
    fn from(response: ReadInputRegistersResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u8(response.bytes_number);
        buf.put_slice(response.values.as_slice());
        buf
    }
}

impl From<WriteSingleCoilResponse> for BytesMut {
    fn from(response: WriteSingleCoilResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(response.coil_address);
        buf.put_u16(response.value);
        buf
    }
}

impl From<WriteSingleHoldingRegisterResponse> for BytesMut {
    fn from(response: WriteSingleHoldingRegisterResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(response.register_address);
        buf.put_u16(response.value);
        buf
    }
}

impl From<WriteMultipleCoilsResponse> for BytesMut {
    fn from(response: WriteMultipleCoilsResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(response.first_address);
        buf.put_u16(response.coils_number);
        buf
    }
}

impl From<WriteMultipleHoldingRegistersResponse> for BytesMut {
    fn from(response: WriteMultipleHoldingRegistersResponse) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(response.first_address);
        buf.put_u16(response.registers_number);
        buf
    }
}

impl From<Head> for BytesMut {
    fn from(head: Head) -> Self {
        let mut buf = BytesMut::from(head.mbap);
        buf.put(BytesMut::from(head.function));
        buf
    }
}

impl From<Function> for BytesMut {
    fn from(function: Function) -> Self {
        let code = match function {
            Function::ReadCoils => 0x01,
            Function::ReadDiscreteInputs => 0x02,
            Function::ReadMultipleHoldingRegisters => 0x03,
            Function::ReadInputRegisters => 0x04,
            Function::WriteSingleCoil => 0x05,
            Function::WriteSingleHoldingRegister => 0x06,
            Function::WriteMultipleCoils => 0x0F,
            Function::WriteMultipleHoldingRegisters => 0x10,
        };
        let mut buf = BytesMut::new();
        buf.put_u8(code);
        buf
    }
}

impl From<Mbap> for BytesMut {
    fn from(mbap: Mbap) -> Self {
        let mut buf = BytesMut::new();
        buf.put_u16(mbap.tid);
        buf.put_u16(mbap.pid);
        buf.put_u16(mbap.length);
        buf.put_u8(mbap.uid);
        buf
    }
}

#[cfg(test)]
mod client_decoder_test {
    use crate::{ClientCodec, Request};
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    #[test]
    fn read_coils_request_test() {
        let mut codec = ClientCodec::default();

        let request = Request::read_coils(0x01, 0x01, 0x02, 0x08);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x01, 0x00, 0x02, 0x00, 0x08,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_discrete_inputs_request_test() {
        let mut codec = ClientCodec::default();

        let request = Request::read_discrete(0x01, 0x01, 0x0000, 0x0012);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x00, 0x00, 0x00, 0x12,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_multiple_holding_registers_request_test() {
        let mut codec = ClientCodec::default();

        let request = Request::read_multiple_holding_registers(0x01, 0x01, 0x0000, 0x0003);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0x00, 0x00, 0x00, 0x03,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_input_registers_request_test() {
        let mut codec = ClientCodec::default();
        let request = Request::read_input_registers(0x01, 0x01, 0x0002, 0x0005);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x04, 0x00, 0x02, 0x00, 0x05,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_coil_request_test() {
        let mut codec = ClientCodec::default();
        let request = Request::write_single_coil(0x01, 0x01, 0x0003, 0xFF00);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0x03, 0xFF, 0x00,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_holding_register_request_test() {
        let mut codec = ClientCodec::default();
        let request = Request::write_single_holding_register(0x01, 0x01, 0x0000, 0x000A);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x00, 0x00, 0x0A,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_coils_request_test() {
        let mut codec = ClientCodec::default();
        let request = Request::write_multiple_coils(0x01, 0x01, 0x001B, 0x0009, vec![0x4D, 0x01]);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D,
            0x01,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_holding_registers_request_test() {
        let mut codec = ClientCodec::default();
        let request =
            Request::write_multiple_holding_registers(0x01, 0x01, 0x0000, vec![0x00, 0x0F]);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x10, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00,
            0x0F,
        ];
        assert_eq!(vec_l, vec_r);
    }
}

#[cfg(test)]
mod server_decoder_test {
    use crate::{Response, ServerCodec};
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    #[test]
    fn read_coils_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::read_coils(0x01, 0x01, vec![0x00, 0x01]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01, 0x01, 0x02, 0x00, 0x01,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_discrete_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::read_discrete(0x01, 0x01, 28, vec![0x01, 0x04, 0x00]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x03, 0x01, 0x04, 0x00,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_holding_register_response_test() {
        let mut codec = ServerCodec::default();
        let response =
            Response::read_holding_register(0x01, 0x01, vec![0x00, 0x21, 0x00, 0x00, 0x00, 0x00]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x03, 0x06, 0x00, 0x21, 0x00, 0x00, 0x00,
            0x00,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_input_register_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::read_input_register(
            0x01,
            0x01,
            vec![0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        );
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x0D, 0x01, 0x04, 0x0A, 0x00, 0x0C, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_coil_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::write_single_coil(0x01, 0x01, 0x0003, 0xFF00);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x05, 0x00, 0x03, 0xFF, 0x00,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_holding_register_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::write_single_holding_register(0x01, 0x01, 0x0000, 0x000A);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x06, 0x00, 0x00, 0x00, 0x0A,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_coils_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::write_multiple_coils(0x01, 0x01, 0x001B, 0x0009);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x0F, 0x00, 0x1B, 0x00, 0x09,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_holding_registers_response_test() {
        let mut codec = ServerCodec::default();
        let response = Response::write_multiple_holding_registers(0x01, 0x01, 0x0000, 0x0001);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x10, 0x00, 0x00, 0x00, 0x01,
        ];
        assert_eq!(vec_l, vec_r);
    }
}
