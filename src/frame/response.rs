use crate::frame::Exception;

use super::{Head, Length};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response {
    ReadCoils(Head, ReadCoilsResponse),
    ReadDiscreteInputs(Head, ReadDiscreteInputsResponse),
    ReadMultipleHoldingRegisters(Head, ReadMultipleHoldingRegistersResponse),
    ReadInputRegisters(Head, ReadInputRegistersResponse),
    WriteSingleCoil(Head, WriteSingleCoilResponse),
    WriteSingleHoldingRegister(Head, WriteSingleHoldingRegisterResponse),
    WriteMultipleCoils(Head, WriteMultipleCoilsResponse),
    WriteMultipleHoldingRegisters(Head, WriteMultipleHoldingRegistersResponse),
    Exception(Head, ExceptionResponse),
}

/// 1
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadCoilsResponse {
    pub(crate) bytes_number: u8,
    pub(crate) values: Vec<u8>,
}

impl Length for ReadCoilsResponse {
    fn len(&self) -> u16 {
        1 + self.values.len() as u16
    }
}

impl ReadCoilsResponse {
    pub(crate) fn new(values: Vec<u8>) -> ReadCoilsResponse {
        let bytes_number = values.len() as u8;
        ReadCoilsResponse {
            bytes_number,
            values,
        }
    }
}

/// 2
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadDiscreteInputsResponse {
    pub(crate) bytes_number: u8,
    pub(crate) values: Vec<u8>,
}

impl Length for ReadDiscreteInputsResponse {
    fn len(&self) -> u16 {
        1 + self.values.len() as u16
    }
}

impl ReadDiscreteInputsResponse {
    pub(crate) fn new(values: Vec<u8>) -> ReadDiscreteInputsResponse {
        ReadDiscreteInputsResponse {
            bytes_number: values.len() as u8,
            values,
        }
    }
}

/// 3
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadMultipleHoldingRegistersResponse {
    pub(crate) bytes_number: u8,
    pub(crate) values: Vec<u8>,
}

impl Length for ReadMultipleHoldingRegistersResponse {
    fn len(&self) -> u16 {
        1 + self.values.len() as u16
    }
}

impl ReadMultipleHoldingRegistersResponse {
    pub(crate) fn new(values: Vec<u8>) -> ReadMultipleHoldingRegistersResponse {
        let bytes_number = values.len() as u8;
        ReadMultipleHoldingRegistersResponse {
            bytes_number,
            values,
        }
    }
}

/// 4
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadInputRegistersResponse {
    pub(crate) bytes_number: u8,
    pub(crate) values: Vec<u8>,
}

impl Length for ReadInputRegistersResponse {
    fn len(&self) -> u16 {
        1 + self.values.len() as u16
    }
}

impl ReadInputRegistersResponse {
    pub(crate) fn new(values: Vec<u8>) -> ReadInputRegistersResponse {
        let bytes_number = values.len() as u8;
        ReadInputRegistersResponse {
            bytes_number,
            values,
        }
    }
}

/// 5
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleCoilResponse {
    pub(crate) coil_address: u16,
    pub(crate) value: u16,
}

impl Length for WriteSingleCoilResponse {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteSingleCoilResponse {
    pub(crate) fn new(coil_address: u16, value: u16) -> WriteSingleCoilResponse {
        WriteSingleCoilResponse {
            coil_address,
            value,
        }
    }
}

/// 6
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleHoldingRegisterResponse {
    pub(crate) register_address: u16,
    pub(crate) value: u16,
}

impl Length for WriteSingleHoldingRegisterResponse {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteSingleHoldingRegisterResponse {
    pub(crate) fn new(register_address: u16, value: u16) -> WriteSingleHoldingRegisterResponse {
        WriteSingleHoldingRegisterResponse {
            register_address,
            value,
        }
    }
}

/// 15
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleCoilsResponse {
    pub(crate) first_address: u16,
    pub(crate) coils_number: u16,
}

impl Length for WriteMultipleCoilsResponse {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteMultipleCoilsResponse {
    pub(crate) fn new(first_address: u16, coils_number: u16) -> WriteMultipleCoilsResponse {
        WriteMultipleCoilsResponse {
            first_address,
            coils_number,
        }
    }
}

/// 16
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleHoldingRegistersResponse {
    pub(crate) first_address: u16,
    pub(crate) registers_number: u16,
}

impl Length for WriteMultipleHoldingRegistersResponse {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteMultipleHoldingRegistersResponse {
    pub(crate) fn new(
        first_address: u16,
        registers_number: u16,
    ) -> WriteMultipleHoldingRegistersResponse {
        WriteMultipleHoldingRegistersResponse {
            first_address,
            registers_number,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExceptionResponse {
    pub(crate) exception: Exception,
}

impl Length for ExceptionResponse {
    fn len(&self) -> u16 {
        1
    }
}

impl ExceptionResponse {
    pub(crate) fn new(exception: Exception) -> Self {
        ExceptionResponse { exception }
    }
}

#[test]
fn test_read_coils_response() {
    let response_l =
        ReadCoilsResponse::new(vec![0b1100_1101, 0b0110_1011, 0b1011_0010, 0b0111_1111]);
    let response_r = ReadCoilsResponse {
        bytes_number: 0x04,
        values: vec![0b1100_1101, 0b0110_1011, 0b1011_0010, 0b0111_1111],
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 5);
}

#[test]
fn test_read_discrete_inputs_response() {
    let response_l =
        ReadDiscreteInputsResponse::new(vec![0b1010_1100, 0b1101_1011, 0b1111_1011, 0b0000_1101]);
    let response_r = ReadDiscreteInputsResponse {
        bytes_number: 0x04,
        values: vec![0b1010_1100, 0b1101_1011, 0b1111_1011, 0b0000_1101],
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 5);
}

#[test]
fn test_read_multiple_holding_registers_response() {
    let response_l =
        ReadMultipleHoldingRegistersResponse::new(vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40]);
    let response_r = ReadMultipleHoldingRegistersResponse {
        bytes_number: 0x06,
        values: vec![0xAE, 0x41, 0x56, 0x52, 0x43, 0x40],
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 7);
}

#[test]
fn test_read_input_register_response() {
    let response_l = ReadInputRegistersResponse::new(vec![0x0C, 0x00, 0x00, 0x00]);
    let response_r = ReadInputRegistersResponse {
        bytes_number: 0x04,
        values: vec![0x0C, 0x00, 0x00, 0x00],
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 5);
}

#[test]
fn test_write_single_coils_response() {
    let response_l = WriteSingleCoilResponse::new(0x00, 0xFF);
    let response_r = WriteSingleCoilResponse {
        coil_address: 0x00,
        value: 0xFF,
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 4);
}

#[test]
fn test_write_single_holding_register_response() {
    let response_l = WriteSingleHoldingRegisterResponse::new(0x01, 0xABCD);
    let response_r = WriteSingleHoldingRegisterResponse {
        register_address: 0x01,
        value: 0xABCD,
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 4);
}

#[test]
fn test_write_multiple_coils_response() {
    let response_l = WriteMultipleCoilsResponse::new(0x00, 0x09);
    let response_r = WriteMultipleCoilsResponse {
        first_address: 0x00,
        coils_number: 0x09,
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 4);
}

#[test]
fn test_multiple_holding_registers_response() {
    let response_l = WriteMultipleHoldingRegistersResponse::new(0x00, 0x02);
    let response_r = WriteMultipleHoldingRegistersResponse {
        first_address: 0x00,
        registers_number: 0x02,
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 4);
}

#[test]
fn test_exception_response() {
    let response_l = ExceptionResponse::new(Exception::IllegalDataAddress);
    let response_r = ExceptionResponse {
        exception: Exception::IllegalDataAddress,
    };
    assert_eq!(response_l, response_r);
    assert_eq!(response_l.len(), 1);
}
