use core::fmt::Debug;

use crate::core::reader::{WasmReadable, WasmReader};
use crate::execution::assert_validated::UnwrapValidatedExt;

#[derive(Debug)]
pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

impl WasmReadable for MemArg {
    fn read(wasm: &mut WasmReader) -> crate::Result<Self> {
        let /* mut  */flags = wasm.read_var_u32()?;

        // let mem = 
        //     // if multi_memory_is_enabled  && flags & (1 << 6) != 0 {
        //     //     flags ^= 1 << 6;
        //     //     wasm.read_var_u32()?
        //     // } else {
        //         0
        //     // }
        //     ;
        let align = if flags >= (1 << 6) {
                panic!("malformed memop alignment: alignment too large")
            } else {
                flags as u32
            };
        let offset = wasm.read_var_u32()?;
        Ok(Self { offset, align })
    }

    fn read_unvalidated(wasm: &mut WasmReader) -> Self {
        let align = wasm.read_var_u32().unwrap_validated();
        let offset = wasm.read_var_u32().unwrap_validated();
        Self { offset, align }
    }
}
