use core::fmt::{Debug, Formatter};

use alloc::{format, vec::Vec};

use crate::core::reader::span::Span;

// use super::opcode::I32_CONST;

pub struct DataSegment {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

#[derive(Clone)]
pub enum DataMode {
    Passive,
    Active(DataModeActive),
}

#[derive(Clone)]
pub struct DataModeActive {
    pub memory_idx: u32,
    pub offset: Span,
}

impl Debug for DataSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut init_str = alloc::string::String::new();

        let mut iter = self.init.iter().peekable();
        // only if it's valid do we print is as a normal utf-8 char, otherwise, hex
        while let Some(&byte) = iter.next() {
            if let Ok(valid_char) = alloc::string::String::from_utf8(Vec::from(&[byte])) {
                init_str.push_str(valid_char.as_str());
            } else {
                init_str.push_str(&format!("\\x{:02x}", byte));
            }
        }

        f.debug_struct("DataSegment")
            .field("init", &init_str)
            .field("mode", &self.mode)
            .finish()
    }
}

/// let final_offset = {
///     active_data_mode.offset.len() == 3 && active_data_mode.offset == I32_CONST {
/// 
///     }
/// }
/// 
///  Usually, we'd have something like this:
/// ```wasm
/// (module
///  (memory 1) ;; memory starting with 1 page
///  (data (i32.const 0) "wtf")  ;; writing the array of byte "wtf" in the first memory (0) at offset 0
///                             ;; for hardcoded offsets, we'll usually use i32.const because of wasm being x86 arch
/// )
/// ```

/// 
/// Since the span has only the start and length and acts a reference, we print the start and end (both inclusive, notice the '..=')
/// We print it in both decimal and hexadecimal so it's easy to trace in something like https://webassembly.github.io/wabt/demo/wat2wasm/
impl Debug for DataMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DataMode::Passive => f.debug_struct("Passive").finish(),
            DataMode::Active(active_data_mode) => {
                let from = active_data_mode.offset.from;
                let to = active_data_mode.offset.from + active_data_mode.offset.len() - 1;
                f.debug_struct("Active")
                    // .field("offset", format_args!("[{}..={}]", from, to))
                    .field("offset", &format_args!("[{}..={}] (hex = [{:X}..={:X}])", from, to, from, to))
                    .finish()
                // f.
            }
        }
    }
}

pub struct PassiveData {
    pub init: Vec<u8>,
}

impl Debug for PassiveData {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut init_str = alloc::string::String::new();

        let mut iter = self.init.iter().peekable();
        while let Some(&byte) = iter.next() {
            if let Ok(valid_char) = alloc::string::String::from_utf8(Vec::from(&[byte])) {
                init_str.push_str(valid_char.as_str());
            } else {
                // If it's not valid UTF-8, print it as hex
                init_str.push_str(&format!("\\x{:02x}", byte));
            }
        }
        f.debug_struct("PassiveData")
            .field("init", &init_str)
            .finish()
    }
}
