#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCodes {
    EOF = 0xFF,
    SELECTDB = 0xFE,
    EXPIRETIME = 0xFD,
    EXPIRETIMEMS = 0xFC,
    RESIZEDB = 0xFB,
    AUX = 0xFA,
}

pub struct OpcodeConversionError;
impl TryFrom<u8> for OpCodes {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xFF => Ok(OpCodes::EOF),
            0xFE => Ok(OpCodes::SELECTDB),
            0xFD => Ok(OpCodes::EXPIRETIME),
            0xFC => Ok(OpCodes::EXPIRETIMEMS),
            0xFB => Ok(OpCodes::RESIZEDB),
            0xFA => Ok(OpCodes::AUX),
            _ => Err(OpcodeConversionError),
        }
    }
    type Error = OpcodeConversionError;
}
