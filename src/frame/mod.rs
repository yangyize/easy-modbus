use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Mutex;

use crate::frame::request::*;
use crate::frame::response::*;

pub mod request;
pub mod response;

/// Modbus Frame
#[derive(Debug)]
pub struct Frame {
    protocol: Protocol,
    tid_map: Mutex<HashMap<u8, u16>>,
}

impl Frame {
    /// Create a TCP frame
    ///
    /// A Modbus variant used for communications over TCP/IP networks.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let tcp = Frame::tcp();
    /// ```
    pub fn tcp() -> Frame {
        Frame {
            protocol: Protocol::Tcp,
            tid_map: Mutex::new(HashMap::new()),
        }
    }

    /// Create a RTU frame
    ///
    /// Used in serial communication, and is the most common implementation available for Modbus.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let rut = Frame::rtu();
    /// ```
    pub fn rtu() -> Frame {
        Frame {
            protocol: Protocol::Rtu,
            tid_map: Mutex::new(HashMap::new()),
        }
    }

    /// Create a read coils request (Function Code: 0x01)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first coil to read
    /// * `number` - Number of coils to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_coils_request(0x01, 0x02, 0x08);
    /// ```
    pub fn read_coils_request(&self, unit_id: u8, first_address: u16, number: u16) -> Request {
        let function = Function::ReadCoils;
        let request_body = ReadCoilsRequest::new(first_address, number);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::ReadCoils(head, request_body)
    }

    /// Create a read discrete Request (Function Code: 0x02)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first discrete input to read
    /// * `number` - Number of discrete input to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_discrete_request(0x0B, 0x007A, 0x001C);
    /// ```
    pub fn read_discrete_request(&self, unit_id: u8, first_address: u16, number: u16) -> Request {
        let function = Function::ReadDiscreteInputs;
        let request_body = ReadDiscreteInputsRequest::new(first_address, number);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::ReadDiscreteInputs(head, request_body)
    }

    /// Create a read multiple holding registers request (Function Code: 0x03)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first register to read
    /// * `number` - Number of discrete input to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_multiple_holding_registers_request(0x0B, 0x006F, 0x0003);
    /// ```
    pub fn read_multiple_holding_registers_request(
        &self,
        unit_id: u8,
        first_address: u16,
        number: u16,
    ) -> Request {
        let function = Function::ReadMultipleHoldingRegisters;
        let request_body = ReadMultipleHoldingRegistersRequest::new(first_address, number);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::ReadMultipleHoldingRegisters(head, request_body)
    }

    /// Create a read input registers request (Function Code: 0x04)
    ///
    /// * `unit_id` -  Server address
    /// * `first_address` - Address of first register to read
    /// * `number` - Number of registers to read
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().read_input_registers_request(0x0B, 0x000A, 0x0001);
    /// ```
    pub fn read_input_registers_request(
        &self,
        unit_id: u8,
        first_address: u16,
        number: u16,
    ) -> Request {
        let function = Function::ReadInputRegisters;
        let request_body = ReadInputRegistersRequest::new(first_address, number);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::ReadInputRegisters(head, request_body)
    }

    /// Create a write single coil request (Function Code: 0x05)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of coil to write
    /// * `value` - Value to write. 0 (0x0000) for off, 65,280 (0xFF00) for on.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_single_coil_request(0x0B, 0x00BF, 0x0000);
    /// ```
    pub fn write_single_coil_request(&self, unit_id: u8, address: u16, value: u16) -> Request {
        let function = Function::WriteSingleCoil;
        let request_body = WriteSingleCoilRequest::new(address, value);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::WriteSingleCoil(head, request_body)
    }

    /// Create a write single holding register request (Function Code: 0x06)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of Holding Register to write
    /// * `value` - Value to write
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_single_holding_register_request(0x0B, 0x0004, 0xABCD);
    /// ```
    pub fn write_single_holding_register_request(
        &self,
        unit_id: u8,
        address: u16,
        value: u16,
    ) -> Request {
        let function = Function::WriteSingleHoldingRegister;
        let request_body = WriteSingleHoldingRegisterRequest::new(address, value);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::WriteSingleHoldingRegister(head, request_body)
    }

    /// Create a write multiple coils request (Function Code: 0x0F)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of Holding Register to write
    /// * `coils_number` - Number of coils to write
    /// * `values` - Coil values. Value of each coil is binary (0 for off, 1 for on).
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_multiple_coils_request(
    ///     0x0B,
    ///     0x001B,
    ///     0x0009,
    ///     vec![0x4D, 0x01]
    /// );
    /// ```
    pub fn write_multiple_coils_request(
        &self,
        unit_id: u8,
        address: u16,
        coils_number: u16,
        values: Vec<u8>,
    ) -> Request {
        let function = Function::WriteMultipleCoils;
        let request_body = WriteMultipleCoilsRequest::new(address, coils_number, values);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::WriteMultipleCoils(head, request_body)
    }

    /// Create a write multiple coils request (Function Code: 0x10)
    ///
    /// * `unit_id` -  Server address
    /// * `address` - Address of first holding registers to write
    /// * `values` - New values of holding registers
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let request = Frame::tcp().write_multiple_holding_registers_request(
    ///     0x0B,
    ///     0x0012,
    ///     vec![0x0B, 0x0A, 0xC1, 0x02],
    /// );
    /// ```
    pub fn write_multiple_holding_registers_request(
        &self,
        unit_id: u8,
        address: u16,
        values: Vec<u8>,
    ) -> Request {
        let function = Function::WriteMultipleHoldingRegisters;
        let request_body = WriteMultipleHoldingRegistersRequest::new(address, values);
        let head = self.head(unit_id, function, request_body.len(), false);
        Request::WriteMultipleHoldingRegisters(head, request_body)
    }

    /// Create a read coils response (Function Code: 0x01)
    ///
    /// * `unit_id` -  Server address
    /// * `values` - Coil input values, Values of each coil input is binary (0 for off, 1 for on).
    /// First requested coil input is as least significant bit of first byte in reply. If number of
    /// coils inputs is not a multiple of 8, most significant bits of last byte will be stuffed zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let response = Frame::tcp().read_coils_response(0x0B, vec![0xCD, 0x6B, 0xB2, 0x7F]);
    /// ```
    pub fn read_coils_response(&self, unit_id: u8, values: Vec<u8>) -> Response {
        let function = Function::ReadCoils;
        let response_body = ReadCoilsResponse::new(values);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::ReadCoils(head, response_body)
    }

    /// Create a read coils response (Function Code: 0x01)
    ///
    /// * `unit_id` -  Server address
    /// * `values` - Discrete input values
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_modbus::Frame;
    /// let response = Frame::tcp().read_discrete_response(0x0B, vec![0xAC, 0xDB, 0xFB, 0x0D]);
    /// ```
    pub fn read_discrete_response(&self, unit_id: u8, values: Vec<u8>) -> Response {
        let function = Function::ReadDiscreteInputs;
        let response_body = ReadDiscreteInputsResponse::new(values);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::ReadDiscreteInputs(head, response_body)
    }

    pub fn read_holding_register_response(&self, unit_id: u8, values: Vec<u8>) -> Response {
        let function = Function::ReadMultipleHoldingRegisters;
        let response_body = ReadMultipleHoldingRegistersResponse::new(values);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::ReadMultipleHoldingRegisters(head, response_body)
    }

    pub fn read_input_register_response(&self, unit_id: u8, values: Vec<u8>) -> Response {
        let function = Function::ReadInputRegisters;
        let response_body = ReadInputRegistersResponse::new(values);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::ReadInputRegisters(head, response_body)
    }

    pub fn write_single_coil_response(&self, unit_id: u8, address: u16, value: u16) -> Response {
        let function = Function::WriteSingleCoil;
        let response_body = WriteSingleCoilResponse::new(address, value);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::WriteSingleCoil(head, response_body)
    }

    pub fn write_single_holding_register_response(
        &self,
        unit_id: u8,
        address: u16,
        value: u16,
    ) -> Response {
        let function = Function::WriteSingleHoldingRegister;
        let response_body = WriteSingleHoldingRegisterResponse::new(address, value);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::WriteSingleHoldingRegister(head, response_body)
    }

    pub fn write_multiple_coils_response(
        &self,
        unit_id: u8,
        address: u16,
        number: u16,
    ) -> Response {
        let function = Function::WriteMultipleCoils;
        let response_body = WriteMultipleCoilsResponse::new(address, number);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::WriteMultipleCoils(head, response_body)
    }

    pub fn write_multiple_holding_registers_response(
        &self,
        unit_id: u8,
        address: u16,
        number: u16,
    ) -> Response {
        let function = Function::WriteMultipleHoldingRegisters;
        let response_body = WriteMultipleHoldingRegistersResponse::new(address, number);
        let head = self.head(unit_id, function, response_body.len(), false);
        Response::WriteMultipleHoldingRegisters(head, response_body)
    }

    pub fn exception_response(
        &self,
        unit_id: u8,
        function: Function,
        exception: Exception,
    ) -> Response {
        let response_body = ExceptionResponse::new(exception);
        let head = self.head(unit_id, function, response_body.len(), true);
        Response::Exception(head, response_body)
    }

    fn head(&self, uid: u8, function: Function, body_length: u16, is_exception: bool) -> Head {
        Head::new(
            self.get_tid(uid),
            uid,
            function,
            body_length,
            self.protocol,
            is_exception,
        )
    }

    pub fn get_tid(&self, unit_id: u8) -> u16 {
        if self.protocol == Protocol::Rtu {
            return 0;
        }

        let mut map = self.tid_map.lock().unwrap();
        let value = match map.get(&unit_id) {
            None => 1,
            Some(v) => {
                if v < &0xFFFF {
                    v + 1
                } else {
                    1
                }
            }
        };
        map.insert(unit_id, value);
        value
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    Tcp,
    Rtu,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Head {
    /// Transaction Identifier
    pub(crate) tid: u16,
    /// Protocol Identifier
    pub(crate) pid: u16,
    /// Pack length
    pub(crate) length: u16,
    /// Server address(Tcp) or Slave address(Rtu)
    pub(crate) uid: u8,
    /// modbus Function
    pub(crate) function: Function,
    /// protocol
    pub(crate) protocol: Protocol,
    /// check is exception
    pub(crate) is_exception: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Exception {
    IllegalFunction,
    IllegalDataAddress,
    IllegalDataValue,
    SlaveDeviceFailure,
    Acknowledge,
}

impl Exception {
    pub(crate) fn to_code(&self) -> u8 {
        use Exception::*;
        match self {
            IllegalFunction => 0x01,
            IllegalDataAddress => 0x02,
            IllegalDataValue => 0x03,
            SlaveDeviceFailure => 0x04,
            Acknowledge => 0x05,
        }
    }
    pub(crate) fn from_code(code: u8) -> Option<Exception> {
        use Exception::*;
        let exception = match code {
            0x01 => IllegalDataValue,
            0x02 => IllegalDataAddress,
            0x03 => IllegalDataValue,
            0x04 => SlaveDeviceFailure,
            0x05 => Acknowledge,
            _ => {
                return None;
            }
        };
        Some(exception)
    }
    pub(crate) fn as_error_kind(&self) -> ErrorKind {
        use Exception::*;
        match self {
            IllegalFunction => ErrorKind::Unsupported,
            IllegalDataAddress => ErrorKind::AddrNotAvailable,
            IllegalDataValue => ErrorKind::InvalidData,
            SlaveDeviceFailure => ErrorKind::Interrupted,
            Acknowledge => ErrorKind::WouldBlock,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Function {
    ReadCoils,
    ReadDiscreteInputs,
    ReadMultipleHoldingRegisters,
    ReadInputRegisters,
    WriteSingleCoil,
    WriteSingleHoldingRegister,
    WriteMultipleCoils,
    WriteMultipleHoldingRegisters,
}

trait Length {
    fn len(&self) -> u16;
}

impl Function {
    pub(crate) fn to_code(&self) -> u8 {
        use Function::*;
        match self {
            ReadCoils => 0x01,
            ReadDiscreteInputs => 0x02,
            ReadMultipleHoldingRegisters => 0x03,
            ReadInputRegisters => 0x04,
            WriteSingleCoil => 0x05,
            WriteSingleHoldingRegister => 0x06,
            WriteMultipleCoils => 0x0F,
            WriteMultipleHoldingRegisters => 0x10,
        }
    }
}

impl Head {
    pub fn new(
        tid: u16,
        uid: u8,
        function: Function,
        body_length: u16,
        protocol: Protocol,
        is_exception: bool,
    ) -> Head {
        Head {
            tid,
            pid: 0x00,
            length: body_length + 2,
            uid,
            function,
            protocol,
            is_exception,
        }
    }

    pub fn body_length(&mut self, body_length: u16) {
        self.length = body_length + 2;
    }
}

#[test]
fn test_head() {
    let head_l = Head::new(0x01, 0x02, Function::ReadCoils, 4, Protocol::Tcp, false);
    let head_r = Head {
        tid: 0x01,
        pid: 0x00,
        length: 6,
        function: Function::ReadCoils,
        uid: 0x02,
        protocol: Protocol::Tcp,
        is_exception: false,
    };
    assert_eq!(head_l, head_r);
}
