use core::fmt::{Debug, Formatter};

use alloc::vec::Vec;

use super::RefType;


#[derive(Debug)]
pub struct ElemType {
    pub ttype: RefType,
    // constant expression
    pub init: Vec<Vec<u8>>,
    pub mode: ElemMode,
}

#[derive(Debug)]
pub enum ElemMode {
    Passive,
    Active(ActiveElem),
    Declarative,
}

pub struct ActiveElem {
    pub table: u32,
    pub offset: Vec<u8>,
}

impl Debug for ActiveElem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let final_offset = {
            if self.offset.len() == 3 && self.offset[0] == 65 {
                self.offset[1] as i64
            } else {
                -1
            }
        };
        f.debug_struct("ActiveElem")
            .field("table", &self.table)
            .field(
                "offset",
                if final_offset == -1 {
                    &self.offset
                } else {
                    &final_offset
                },
            )
            .finish()
    }
}