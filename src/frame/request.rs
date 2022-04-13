use super::{Head, Length};

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

/// 1
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadCoilsRequest {
    pub(crate) first_address: u16,
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

/// 2
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadDiscreteInputsRequest {
    pub(crate) first_address: u16,
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

/// 3
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadMultipleHoldingRegistersRequest {
    pub(crate) first_address: u16,
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

/// 4
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadInputRegistersRequest {
    pub(crate) first_address: u16,
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

/// 5
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleCoilRequest {
    pub(crate) coil_address: u16,
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

/// 6
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSingleHoldingRegisterRequest {
    pub(crate) register_address: u16,
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

/// 15
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleCoilsRequest {
    pub(crate) first_address: u16,
    pub(crate) coils_number: u16,
    pub(crate) bytes_number: u8,
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

/// 16
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteMultipleHoldingRegistersRequest {
    pub(crate) first_address: u16,
    pub(crate) registers_number: u16,
    pub(crate) bytes_number: u8,
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
