use std::io::{Error, Result};

use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::codec::{RtuClientCodec, RtuServerCodec, TcpClientCodec};
use crate::frame::request::*;
use crate::frame::response::*;
use crate::frame::response::Response;

use super::TcpServerCodec;

impl Encoder<Request> for RtuClientCodec {
    type Error = Error;

    fn encode(
        &mut self,
        item: Request,
        dst: &mut BytesMut,
    ) -> std::result::Result<(), Self::Error> {
        request_to_bytesmut(item, dst);
        Ok(())
    }
}

impl Encoder<Response> for RtuServerCodec {
    type Error = Error;

    fn encode(
        &mut self,
        item: Response,
        dst: &mut BytesMut,
    ) -> std::result::Result<(), Self::Error> {
        response_to_bytesmut(item, dst);
        Ok(())
    }
}

impl Encoder<Request> for TcpClientCodec {
    type Error = Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<()> {
        request_to_bytesmut(item, dst);
        Ok(())
    }
}

impl Encoder<Response> for TcpServerCodec {
    type Error = Error;

    fn encode(&mut self, item: Response, dst: &mut BytesMut) -> Result<()> {
        response_to_bytesmut(item, dst);
        Ok(())
    }
}

#[cfg(test)]
mod rtu_client_encoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    use crate::codec::RtuClientCodec;
    use crate::frame::Frame;

    #[test]
    fn read_coils_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.read_coils_request(0x0B, 0x001D, 0x001F);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x01, 0x00, 0x1D, 0x00, 0x1F, 0xED, 0x6E];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_discrete_inputs_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.read_discrete_request(0x0B, 0x007A, 0x001C);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x02, 0x00, 0x7A, 0x00, 0x1C, 0x58, 0xB0];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_multiple_holding_registers_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.read_multiple_holding_registers_request(0x0B, 0x006F, 0x0003);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x03, 0x00, 0x6F, 0x00, 0x03, 0x35, 0x7C];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_input_registers_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.read_input_registers_request(0x0B, 0x000A, 0x0001);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x04, 0x00, 0x0A, 0x00, 0x01, 0x11, 0x62];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_coil_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.write_single_coil_request(0x0B, 0x00BF, 0x0000);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x05, 0x00, 0xBF, 0x00, 0x00, 0xFC, 0x84];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_holding_register_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.write_single_holding_register_request(0x0B, 0x0004, 0xABCD);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x06, 0x00, 0x04, 0xAB, 0xCD, 0x76, 0x04];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_coils_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.write_multiple_coils_request(0x0B, 0x001B, 0x0009, vec![0x4D, 0x01]);
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x0B, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0x02, 0x4D, 0x01, 0x6C, 0xA7,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_holding_registers_request_test() {
        let mut codec = RtuClientCodec::default();
        let frame = Frame::rtu();
        let request = frame.write_multiple_holding_registers_request(
            0x0B,
            0x0012,
            vec![0x0B, 0x0A, 0xC1, 0x02],
        );
        let mut dst = BytesMut::new();
        let res = codec.encode(request, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x0B, 0x10, 0x00, 0x12, 0x00, 0x02, 0x04, 0x0B, 0x0A, 0xC1, 0x02, 0xA0, 0xD5,
        ];
        assert_eq!(vec_l, vec_r);
    }
}

#[cfg(test)]
mod tcp_client_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    use crate::codec::TcpClientCodec;
    use crate::frame::Frame;

    #[test]
    fn read_coils_request_test() {
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.read_coils_request(0x01, 0x02, 0x08);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.read_discrete_request(0x01, 0x0000, 0x0012);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.read_multiple_holding_registers_request(0x01, 0x0000, 0x0003);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.read_input_registers_request(0x01, 0x0002, 0x0005);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.write_single_coil_request(0x01, 0x0003, 0xFF00);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.write_single_holding_register_request(0x01, 0x0000, 0x000A);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request = frame.write_multiple_coils_request(0x01, 0x001B, 0x0009, vec![0x4D, 0x01]);
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
        let mut codec = TcpClientCodec::default();
        let frame = Frame::tcp();
        let request =
            frame.write_multiple_holding_registers_request(0x01, 0x0000, vec![0x00, 0x0F]);
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
mod tcp_server_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    use crate::{codec::TcpServerCodec, Frame};
    use crate::frame::{Exception, Function};

    #[test]
    fn read_coils_response_test() {
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.read_coils_response(0x01, vec![0x00, 0x01]);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.read_discrete_response(0x01, vec![0x01, 0x04, 0x00]);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response =
            frame.read_holding_register_response(0x01, vec![0x00, 0x21, 0x00, 0x00, 0x00, 0x00]);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.read_input_register_response(
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.write_single_coil_response(0x01, 0x0003, 0xFF00);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.write_single_holding_register_response(0x01, 0x0000, 0x000A);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.write_multiple_coils_response(0x01, 0x001B, 0x0009);
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
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response = frame.write_multiple_holding_registers_response(0x01, 0x0000, 0x0001);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x10, 0x00, 0x00, 0x00, 0x01,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn exception_response_test() {
        let mut codec = TcpServerCodec::default();
        let frame = Frame::tcp();
        let response =
            frame.exception_response(0x0A, Function::ReadCoils, Exception::IllegalDataAddress);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x0A, 0x81, 0x02];
        assert_eq!(vec_l, vec_r);
    }
}

#[cfg(test)]
mod rtu_server_decoder_test {
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    use crate::{codec::RtuServerCodec, Frame};
    use crate::frame::{Exception, Function};

    #[test]
    fn read_coils_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.read_coils_response(0x0B, vec![0xCD, 0x6B, 0xB2, 0x7F]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x01, 0x04, 0xCD, 0x6B, 0xB2, 0x7F, 0x2B, 0xE1];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_discrete_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.read_discrete_response(0x0B, vec![0xAC, 0xDB, 0xFB, 0x0D]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x02, 0x04, 0xAC, 0xDB, 0xFB, 0x0D, 0x82, 0x7C];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_holding_register_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response =
            frame.read_holding_register_response(0x0B, vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![
            0x0B, 0x03, 0x06, 0xAE, 0x41, 0x56, 0x52, 0x43, 0x40, 0xFA, 0xCD,
        ];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn read_input_register_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.read_input_register_response(0x0B, vec![0x10, 0x2F]);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x04, 0x02, 0x10, 0x2F, 0x6D, 0x2D];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_coil_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.write_single_coil_response(0x0B, 0x00BF, 0x0000);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x05, 0x00, 0xBF, 0x00, 0x00, 0xFC, 0x84];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_single_holding_register_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.write_single_holding_register_response(0x0B, 0x0004, 0xABCD);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x006, 0x000, 0x004, 0x0AB, 0x0CD, 0x076, 0x004];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_coils_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.write_multiple_coils_response(0x0B, 0x001B, 0x0009);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x0F, 0x00, 0x1B, 0x00, 0x09, 0xE5, 0x60];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn write_multiple_holding_registers_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response = frame.write_multiple_holding_registers_response(0x0B, 0x0012, 0x0002);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0B, 0x10, 0x00, 0x12, 0x00, 0x02, 0xE1, 0x67];
        assert_eq!(vec_l, vec_r);
    }

    #[test]
    fn exception_response_test() {
        let mut codec = RtuServerCodec::default();
        let frame = Frame::rtu();
        let response =
            frame.exception_response(0x0A, Function::ReadCoils, Exception::IllegalDataAddress);
        let mut dst = BytesMut::new();
        let res = codec.encode(response, &mut dst);
        assert!(res.is_ok());
        let vec_l = dst.to_vec();
        let vec_r = vec![0x0A, 0x81, 0x02, 0xB0, 0x53];
        assert_eq!(vec_l, vec_r);
    }
}
