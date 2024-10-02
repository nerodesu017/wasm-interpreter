use crate::core::reader::span::Span;
use crate::core::reader::{WasmReadable, WasmReader};
use crate::{Error, RefType, Result};

pub(super) fn read_constant_expression(wasm: &mut WasmReader) -> Result<Span> {
    let start_pc = wasm.pc;

    loop {
        let Ok(first_instr_byte) = wasm.read_u8() else {
            return Err(Error::ExprMissingEnd);
        };
        trace!("Read constant instruction byte {first_instr_byte:#X?} ({first_instr_byte})");

        use crate::core::reader::types::opcode::*;
        match first_instr_byte {
            END => {
                return Ok(Span::new(start_pc, wasm.pc - start_pc + 1));
            }
            GLOBAL_GET => {
                wasm.read_var_u32()?;
            }
            I32_CONST => {
                wasm.read_var_i32()?;
            }
            I64_CONST => {
                wasm.read_var_i64()?;
            }
            I32_ADD | I32_SUB | I32_MUL => {}
            I64_ADD | I64_SUB | I64_MUL => {}
            REF_NULL => {
                RefType::read(wasm).unwrap();
            }
            REF_FUNC => {
                wasm.read_var_u32().unwrap();
            }

            _ => return Err(Error::InvalidInstr(first_instr_byte)),
        }
    }
}
