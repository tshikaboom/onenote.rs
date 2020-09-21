use crate::Reader;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct JcId(pub(crate) u32);

impl JcId {
    pub(crate) fn parse(reader: Reader) -> JcId {
        JcId(reader.get_u32_le())
    }
}

impl fmt::Debug for JcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JcId(0x{:08X})", self.0)
    }
}
