pub mod request;
pub mod response;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Head {
    pub(crate) mbap: Mbap,
    pub(crate) function: Function,
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Mbap {
    /// Transaction Identifier
    pub(crate) tid: u16,
    /// Protocol Identifier
    pub(crate) pid: u16,
    /// Pack length
    pub(crate) length: u16,
    /// Server address
    pub(crate) uid: u8,
}

impl Mbap {
    pub fn new(tid: u16, uid: u8, body_length: u16) -> Mbap {
        Mbap {
            tid,
            pid: 0x00,
            length: body_length + 2,
            uid,
        }
    }
}

impl Head {
    pub fn new(tid: u16, uid: u8, function: Function, body_length: u16) -> Head {
        Head {
            mbap: Mbap::new(tid, uid, body_length),
            function,
        }
    }
}

#[test]
fn test_mbap() {
    let mbap_l = Mbap::new(0x01, 0x02, 4);
    let mbap_r = Mbap {
        tid: 0x01,
        pid: 0x00,
        length: 6,
        uid: 0x02,
    };
    assert_eq!(mbap_l, mbap_r);
}

#[test]
fn test_head() {
    let head_l = Head::new(0x01, 0x02, Function::ReadCoils, 4);

    let mbap = Mbap::new(0x01, 0x02, 4);
    let head_r = Head {
        mbap,
        function: Function::ReadCoils,
    };
    assert_eq!(head_l, head_r);
}
