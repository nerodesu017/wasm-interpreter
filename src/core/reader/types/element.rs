use core::fmt::{Debug, Display};

use alloc::vec::Vec;

use crate::core::reader::span::Span;

use super::RefType;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ElemType {
    pub init: ElemItems,
    pub mode: ElemMode,
}

impl Debug for ElemType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ElemType {{\n\tinit: {:?},\n\tmode: {:?},\n\t#ty: {}\n}}",
            self.init, self.mode, self.init.ty()
        )
    }
}

impl ElemType {
    pub fn ty(&self) -> RefType {
        self.init.ty()
    }
    pub fn to_ref_type(&self) -> RefType {
        match self.init {
            ElemItems::Exprs(rref, _) => rref.clone(),
            ElemItems::RefFuncs(_) => RefType::FuncRef,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElemItems {
    RefFuncs(Vec<u32>),
    Exprs(RefType, Vec<Span>),
}

impl ElemItems {
    pub fn ty(&self) -> RefType {
        match self {
            Self::RefFuncs(_) => RefType::FuncRef,
            Self::Exprs(rty, _) => rty.clone(),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::RefFuncs(v) => v.len(),
            Self::Exprs(_, v) => v.len(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ElemMode {
    Passive,
    Active(ActiveElem),
    Declarative,
}

#[derive(Debug, Clone)]
pub struct ActiveElem {
    pub table: u32,
    pub offset: Span, //Vec<u8>,
}

// impl Debug for ActiveElem {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         let final_offset = {
//             if self.offset.len() == 3 && self.offset[0] == 65 {
//                 self.offset[1] as i64
//             } else {
//                 -1
//             }
//         };
//         f.debug_struct("ActiveElem")
//             .field("table", &self.table)
//             .field(
//                 "offset",
//                 if final_offset == -1 {
//                     &self.offset
//                 } else {
//                     &final_offset
//                 },
//             )
//             .finish()
//     }
// }
