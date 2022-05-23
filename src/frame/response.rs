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

/// Function Code `0x01`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadCoilsResponse {
    /// Number of bytes of coil input values to follow
    pub(crate) bytes_number: u8,

    /// Coil input values
    ///
    /// Values of each coil input is binary (0 for off, 1 for on). First requested coil input is
    /// as least significant bit of first byte in reply. If number of coils inputs is not a multiple
    /// of 8, most significant bits of last byte will be stuffed zeros.
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

/// Function Code `0x02`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadDiscreteInputsResponse {
    /// Number of bytes of discrete input values to follow
    pub(crate) bytes_number: u8,

    /// Discrete input values
    ///
    /// Values of each discrete input is binary (0 for off, 1 for on). First requested discrete
    /// input is stored as least significant bit of first byte in reply. If number of discrete
    /// inputs is not a multiple of 8, most significant bits of last byte will be stuffed with
    /// zeros.
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

/// Function Code `0x03`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadMultipleHoldingRegistersResponse {
    /// Number of bytes of register values to follow
    pub(crate) bytes_number: u8,

    /// Register values
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

/// Function Code `0x04`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadInputRegistersResponse {
    /// Number of bytes of register value to follow
    pub(crate) bytes_number: u8,

    /// Register values
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

/// Function Code `0x05`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleCoilResponse {
    /// Address of coil
    pub(crate) coil_address: u16,

    /// Value to write
    ///
    /// 0 (0x0000) for off, 65,280 (0xFF00) for on
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

/// Function Code `0x06`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleHoldingRegisterResponse {
    /// Address of holding register to write
    pub(crate) register_address: u16,

    /// Value to write
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

/// Function Code `0x15`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleCoilsResponse {
    /// Address of the first coil
    pub(crate) first_address: u16,

    /// Number of coils
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

/// Function Code `0x16`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleHoldingRegistersResponse {
    /// Address of first written holding register
    pub(crate) first_address: u16,

    /// Number of written holding registers
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

#[cfg(test)]
mod response_test {
    use crate::frame::{Exception, Length};
    use crate::frame::response::*;

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
}