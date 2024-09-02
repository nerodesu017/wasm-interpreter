use alloc::vec::Vec;

use crate::core::indices::{FuncIdx, TypeIdx};
use crate::core::reader::section_header::{SectionHeader, SectionTy};
use crate::core::reader::span::Span;
use crate::core::reader::types::export::Export;
use crate::core::reader::types::global::Global;
use crate::core::reader::types::import::Import;
use crate::core::reader::types::{FuncType, MemType, TableType};
use crate::core::reader::{WasmReadable, WasmReader};
use crate::{Error, Result};

pub(crate) mod code;

// const CUSTOM_SECTION: u8 = 0x00;
// const TYPE_SECTION: u8 = 0x01;
// const IMPORT_SECTION: u8 = 0x02;
// const FUNCTION_SECTION: u8 = 0x03;
// const TABLE_SECTION: u8 = 0x04;
// const MEMORY_SECTION: u8 = 0x05;
// const GLOBAL_SECTION: u8 = 0x06;
// const EXPORT_SECTION: u8 = 0x07;
// const START_SECTION: u8 = 0x08;
// const ELEMENT_SECTION: u8 = 0x09;
// const CODE_SECTION: u8 = 0x0A;
// const DATA_SECTION: u8 = 0x0B;
// const DATA_COUNT_SECTION: u8 = 0x0C;
// const TAG_SECTION: u8 = 0x0D;

#[derive(Debug)]
pub(crate) enum WasmType {
    Module,
    Component,
}

/// Information collected from validating a module.
/// This can be used to create a [crate::RuntimeInstance].
pub struct ValidationInfo<'bytecode> {
    pub(crate) wasm: &'bytecode [u8],
    pub(crate) types: Vec<FuncType>,
    #[allow(dead_code)]
    pub(crate) imports: Vec<Import>,
    pub(crate) functions: Vec<TypeIdx>,
    #[allow(dead_code)]
    pub(crate) tables: Vec<TableType>,
    pub(crate) memories: Vec<MemType>,
    pub(crate) globals: Vec<Global>,
    #[allow(dead_code)]
    pub(crate) exports: Vec<Export>,
    pub(crate) func_blocks: Vec<Span>,
    /// The start function which is automatically executed during instantiation
    pub(crate) start: Option<FuncIdx>,
    #[allow(dead_code)]
    pub(crate) bytes_type: WasmType,
}

pub fn validate(wasm: &[u8]) -> Result<ValidationInfo> {
    let mut wasm = WasmReader::new(wasm);
    trace!("Starting validation of bytecode");

    trace!("Validating magic value");
    let [0x00, 0x61, 0x73, 0x6d] = wasm.strip_bytes::<4>()? else {
        return Err(Error::InvalidMagic);
    };

    trace!("Validating version number");
    let bytes_type = match wasm.strip_bytes::<4>()? {
        [0x01, 0x00, 0x00, 0x00] => WasmType::Module,
        [0x0D, 0x00, 0x01, 0x00] => WasmType::Component,
        bytes => {
            trace!(
                "Unknown version: {:X} {:X} {:X} {:X}",
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3]
            );
            return Err(Error::InvalidMagic);
        }
    };
    debug!("Header ok");
    trace!("WasmType: {:?}", bytes_type);

    let mut header = None;
    read_next_header(&mut wasm, &mut header)?;

    let skip_section = |wasm: &mut WasmReader, section_header: &mut Option<SectionHeader>| {
        handle_section(wasm, section_header, SectionTy::Custom, |wasm, h| {
            wasm.skip(h.contents.len())
        })
    };

    let mut types = Vec::new();
    let mut imports = Vec::new();
    let mut functions = Vec::new();
    let mut tables = Vec::new();
    let mut memories = Vec::new();
    let mut globals = Vec::new();
    let mut exports = Vec::new();
    let mut start = None;
    let mut func_blocks = Vec::new();

    while let Some(inner_header) = header.take() {
        let ty = inner_header.ty;
        header = Some(inner_header);
        trace!("Found a {:?} section", ty);
        match ty {
            SectionTy::Custom => {
                handle_section(&mut wasm, &mut header, SectionTy::Custom, |wasm, h| {
                    wasm.skip(h.contents.len())
                })
                .unwrap_or_default();
            }
            SectionTy::Type => {
                types = handle_section(&mut wasm, &mut header, SectionTy::Type, |wasm, _| {
                    wasm.read_vec(FuncType::read)
                })?
                .unwrap_or_default();
            }
            SectionTy::Import => {
                imports = handle_section(&mut wasm, &mut header, SectionTy::Import, |wasm, _| {
                    wasm.read_vec(Import::read)
                })?
                .unwrap_or_default();
            }
            SectionTy::Function => {
                functions =
                    handle_section(&mut wasm, &mut header, SectionTy::Function, |wasm, _| {
                        wasm.read_vec(|wasm| wasm.read_var_u32().map(|u| u as usize))
                    })?
                    .unwrap_or_default();
            }
            SectionTy::Table => {
                tables = handle_section(&mut wasm, &mut header, SectionTy::Table, |wasm, _| {
                    wasm.read_vec(TableType::read)
                })?
                .unwrap_or_default();
            }
            SectionTy::Memory => {
                memories = handle_section(&mut wasm, &mut header, SectionTy::Memory, |wasm, _| {
                    wasm.read_vec(MemType::read)
                })?
                .unwrap_or_default();
                if memories.len() > 1 {
                    return Err(Error::MoreThanOneMemory);
                }
            }
            SectionTy::Global => {
                globals = handle_section(&mut wasm, &mut header, SectionTy::Global, |wasm, _| {
                    wasm.read_vec(|wasm| {
                        // TODO validate instructions in `global.init_expr`. Furthermore all of these instructions need to be constant.
                        //  See https://webassembly.github.io/spec/core/valid/instructions.html#valid-constant
                        //  Maybe we can also execute constant expressions right here so they do not even exist in the runtime environment. <-- Needs further research to check if this is even possible
                        Global::read(wasm)
                    })
                })?
                .unwrap_or_default();
            }
            SectionTy::Export => {
                exports = handle_section(&mut wasm, &mut header, SectionTy::Export, |wasm, _| {
                    wasm.read_vec(Export::read)
                })?
                .unwrap_or_default();
            }
            SectionTy::Start => {
                start = handle_section(&mut wasm, &mut header, SectionTy::Start, |wasm, _| {
                    wasm.read_var_u32().map(|idx| idx as FuncIdx)
                })?;
            }
            SectionTy::Element => {
                handle_section(
                    &mut wasm,
                    &mut header,
                    SectionTy::Element,
                    |_, _| -> Result<()> {
                        todo!("element section not yet supported");
                    },
                )?;
            }
            SectionTy::DataCount => {
                handle_section(
                    &mut wasm,
                    &mut header,
                    SectionTy::DataCount,
                    |_, _| -> Result<()> {
                        todo!("data count section not yet supported");
                    },
                )?;
            }
            SectionTy::Code => {
                func_blocks =
                    handle_section(&mut wasm, &mut header, SectionTy::Code, |wasm, h| {
                        code::validate_code_section(wasm, h, &types, &functions, &globals)
                    })?
                    .unwrap_or_default();
            }
            SectionTy::Data => {
                handle_section(
                    &mut wasm,
                    &mut header,
                    SectionTy::Data,
                    |_, _| -> Result<()> { todo!("data section not yet supported") },
                )?;
            }
        }

        while (skip_section(&mut wasm, &mut header)?).is_some() {}

        trace!("{:#?}", header);
    }

    assert_eq!(func_blocks.len(), functions.len(), "these should be equal");

    // All sections should have been handled
    if let Some(header) = header {
        return Err(Error::SectionOutOfOrder(header.ty));
    }

    debug!("Validation was successful");
    Ok(ValidationInfo {
        wasm: wasm.into_inner(),
        types,
        imports,
        functions,
        tables,
        memories,
        globals,
        exports,
        func_blocks,
        start,
        bytes_type,
    })
}

fn read_next_header(wasm: &mut WasmReader, header: &mut Option<SectionHeader>) -> Result<()> {
    if header.is_none() && !wasm.remaining_bytes().is_empty() {
        *header = Some(SectionHeader::read(wasm)?);
    }
    Ok(())
}

#[inline(always)]
fn handle_section<T, F: FnOnce(&mut WasmReader, SectionHeader) -> Result<T>>(
    wasm: &mut WasmReader,
    header: &mut Option<SectionHeader>,
    section_ty: SectionTy,
    handler: F,
) -> Result<Option<T>> {
    match &header {
        Some(SectionHeader { ty, .. }) if *ty == section_ty => {
            let h = header.take().unwrap();
            trace!("Handling section {:?}", h.ty);
            let ret = handler(wasm, h)?;
            read_next_header(wasm, header)?;
            Ok(Some(ret))
        }
        _ => Ok(None),
    }
}
