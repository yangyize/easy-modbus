use super::{Head, Length};

/// Modbus Request
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Request {
    ReadCoils(Head, ReadCoilsRequest),
    ReadDiscreteInputs(Head, ReadDiscreteInputsRequest),
    ReadMultipleHoldingRegisters(Head, ReadMultipleHoldingRegistersRequest),
    ReadInputRegisters(Head, ReadInputRegistersRequest),
    WriteSingleCoil(Head, WriteSingleCoilRequest),
    WriteSingleHoldingRegister(Head, WriteSingleHoldingRegisterRequest),
    WriteMultipleCoils(Head, WriteMultipleCoilsRequest),
    WriteMultipleHoldingRegisters(Head, WriteMultipleHoldingRegistersRequest),
}

/// Function Code `0x01`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadCoilsRequest {
    /// Address of first coil to read
    pub(crate) first_address: u16,

    /// Number of coils to read
    ///
    /// Because of the byte count returned in the reply message is only 8 bits wide and the protocol
    /// overhead is 5 bytes, a maximum of 2008(251 * 8) discrete coils can be read at once.
    pub(crate) coils_number: u16,
}

impl Length for ReadCoilsRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl ReadCoilsRequest {
    pub(crate) fn new(first_address: u16, coils_number: u16) -> ReadCoilsRequest {
        ReadCoilsRequest {
            first_address,
            coils_number,
        }
    }
}

/// Function Code `0x02`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadDiscreteInputsRequest {
    /// Address of first discrete input to read
    pub(crate) first_address: u16,

    /// Number of discrete input to read
    ///
    /// Because of the byte count returned in the reply message is only 8 bits wide and the protocol
    /// overhead is 5 bytes, a maximum of 2008(251 * 8) discrete inputs can be read at once.
    pub(crate) discrete_inputs_number: u16,
}

impl Length for ReadDiscreteInputsRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl ReadDiscreteInputsRequest {
    pub(crate) fn new(
        first_address: u16,
        discrete_inputs_number: u16,
    ) -> ReadDiscreteInputsRequest {
        ReadDiscreteInputsRequest {
            first_address,
            discrete_inputs_number,
        }
    }
}

/// Function Code `0x03`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadMultipleHoldingRegistersRequest {
    /// Address of first register to read
    pub(crate) first_address: u16,

    /// Number of registers to read
    ///
    /// Because the maximum length of a Modbus PDU is 253, so up to 125 registers can be requested
    /// at once when using RTU format, and up to 123 over TCP
    pub(crate) registers_number: u16,
}

impl Length for ReadMultipleHoldingRegistersRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl ReadMultipleHoldingRegistersRequest {
    pub(crate) fn new(
        first_address: u16,
        registers_number: u16,
    ) -> ReadMultipleHoldingRegistersRequest {
        ReadMultipleHoldingRegistersRequest {
            first_address,
            registers_number,
        }
    }
}

/// Function code `0x04`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadInputRegistersRequest {
    /// Address of first register to read
    pub(crate) first_address: u16,

    /// Number of registers to read
    ///
    /// Because the maximum length of a Modbus PDU is 253, so up to 125 registers can be requested
    /// at once when using RTU format, and up to 123 over TCP
    pub(crate) registers_number: u16,
}

impl Length for ReadInputRegistersRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl ReadInputRegistersRequest {
    pub(crate) fn new(first_address: u16, registers_number: u16) -> ReadInputRegistersRequest {
        ReadInputRegistersRequest {
            first_address,
            registers_number,
        }
    }
}

/// Function Code `0x05`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleCoilRequest {
    /// Address of coil to write
    pub(crate) coil_address: u16,

    /// Value to write
    ///
    /// 0 (0x0000) for off, 65,280 (0xFF00) for on
    pub(crate) value: u16,
}

impl Length for WriteSingleCoilRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteSingleCoilRequest {
    pub(crate) fn new(coil_address: u16, value: u16) -> WriteSingleCoilRequest {
        WriteSingleCoilRequest {
            coil_address,
            value,
        }
    }
}

/// Function Code `0x06`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleHoldingRegisterRequest {
    /// Address of Holding Register to write
    pub(crate) register_address: u16,

    /// Value to write
    pub(crate) value: u16,
}

impl Length for WriteSingleHoldingRegisterRequest {
    fn len(&self) -> u16 {
        4
    }
}

impl WriteSingleHoldingRegisterRequest {
    pub(crate) fn new(register_address: u16, value: u16) -> WriteSingleHoldingRegisterRequest {
        WriteSingleHoldingRegisterRequest {
            register_address,
            value,
        }
    }
}

/// Function Code `0x0F`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleCoilsRequest {
    /// Address of first coil to write
    ///
    /// First requested coil is stored as least significant bit of first byte in request.
    pub(crate) first_address: u16,

    /// Number of coils to write
    pub(crate) coils_number: u16,

    /// Number of bytes of coil values to follow
    ///
    /// If number of coils is not a multiple of 8, most significant bits of last byte should be
    /// stuffed with zeros.
    pub(crate) bytes_number: u8,

    /// Coil values
    ///
    /// Value of each coil is binary (0 for off, 1 for on).
    pub(crate) values: Vec<u8>,
}

impl Length for WriteMultipleCoilsRequest {
    fn len(&self) -> u16 {
        5 + self.values.len() as u16
    }
}

impl WriteMultipleCoilsRequest {
    pub(crate) fn new(
        first_address: u16,
        coils_number: u16,
        values: Vec<u8>,
    ) -> WriteMultipleCoilsRequest {
        WriteMultipleCoilsRequest {
            first_address,
            coils_number,
            bytes_number: values.len() as u8,
            values,
        }
    }
}

/// Function Code `0x10`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleHoldingRegistersRequest {
    /// Address of first holding registers to write
    pub(crate) first_address: u16,

    /// Number of holding registers to write
    ///
    /// Because the maximum length of a Modbus PDU is 253 (inferred from the maximum Modbus APU
    /// length of 256 on RS485), up to 123 registers can be written at once.
    pub(crate) registers_number: u16,

    /// Number of bytes of register value to follow
    pub(crate) bytes_number: u8,

    /// New values of holding registers
    pub(crate) values: Vec<u8>,
}

impl Length for WriteMultipleHoldingRegistersRequest {
    fn len(&self) -> u16 {
        5 + self.values.len() as u16
    }
}

impl WriteMultipleHoldingRegistersRequest {
    pub(crate) fn new(first_address: u16, values: Vec<u8>) -> WriteMultipleHoldingRegistersRequest {
        WriteMultipleHoldingRegistersRequest {
            first_address,
            registers_number: values.len() as u16 / 2,
            bytes_number: values.len() as u8,
            values,
        }
    }
}

#[cfg(test)]
mod request_test {
    use crate::frame::Length;
    use crate::frame::request::*;

    #[test]
    fn test_read_coils_request() {
        let request_l = ReadCoilsRequest::new(0x01, 0x02);
        let request_r = ReadCoilsRequest {
            first_address: 0x01,
            coils_number: 0x02,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_read_discrete_inputs_request() {
        let request_l = ReadDiscreteInputsRequest::new(0x01, 0x02);
        let request_r = ReadDiscreteInputsRequest {
            first_address: 0x01,
            discrete_inputs_number: 0x02,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_read_multiple_holding_register_request() {
        let request_l = ReadMultipleHoldingRegistersRequest::new(0x01, 0x02);
        let request_r = ReadMultipleHoldingRegistersRequest {
            first_address: 0x01,
            registers_number: 0x02,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_read_input_register_request() {
        let request_l = ReadInputRegistersRequest::new(0x01, 0x02);
        let request_r = ReadInputRegistersRequest {
            first_address: 0x01,
            registers_number: 0x02,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_write_single_coil_request() {
        let request_l = WriteSingleCoilRequest::new(0x01, 0xABCD);
        let request_r = WriteSingleCoilRequest {
            coil_address: 0x01,
            value: 0xABCD,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_write_single_holding_register_request() {
        let request_l = WriteSingleHoldingRegisterRequest::new(0x01, 0x02);
        let request_r = WriteSingleHoldingRegisterRequest {
            register_address: 0x01,
            value: 0x02,
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 4);
    }

    #[test]
    fn test_write_multiple_coils_request() {
        let request_l = WriteMultipleCoilsRequest::new(0x01, 0x09, vec![0b0100_1101, 0b0000_0001]);
        let request_r = WriteMultipleCoilsRequest {
            first_address: 0x01,
            coils_number: 0x09,
            bytes_number: 0x02,
            values: vec![0b0100_1101, 0b0000_0001],
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 7);
    }

    #[test]
    fn test_write_multiple_holding_registers_request() {
        let request_l = WriteMultipleHoldingRegistersRequest::new(0x01, vec![0x00, 0x0F]);
        let request_r = WriteMultipleHoldingRegistersRequest {
            first_address: 0x01,
            registers_number: 0x01,
            bytes_number: 0x02,
            values: vec![0x00, 0x0f],
        };
        assert_eq!(request_l, request_r);
        assert_eq!(request_l.len(), 7);
    }
}