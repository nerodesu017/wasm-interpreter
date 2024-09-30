use alloc::vec::{self, Vec};

use crate::core::indices::{FuncIdx, TypeIdx};
use crate::core::reader::section_header::{SectionHeader, SectionTy};
use crate::core::reader::span::Span;
use crate::core::reader::types::export::Export;
use crate::core::reader::types::global::Global;
use crate::core::reader::types::import::Import;
use crate::core::reader::types::{
    opcode, ActiveDataForMemoryX, DataType, FuncType, MemType, PassiveData, TableType,
};
use crate::core::reader::{WasmReadable, WasmReader};
use crate::{Error, Result};

pub(crate) mod code;
pub(crate) mod globals;
pub(crate) mod validation_stack;

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
}

pub fn validate(wasm: &[u8]) -> Result<ValidationInfo> {
    let mut wasm = WasmReader::new(wasm);
    trace!("Starting validation of bytecode");

    trace!("Validating magic value");
    let [0x00, 0x61, 0x73, 0x6d] = wasm.strip_bytes::<4>()? else {
        return Err(Error::InvalidMagic);
    };

    trace!("Validating version number");
    let [0x01, 0x00, 0x00, 0x00] = wasm.strip_bytes::<4>()? else {
        return Err(Error::InvalidVersion);
    };
    debug!("Header ok");

    let mut header = None;
    read_next_header(&mut wasm, &mut header)?;

    let skip_section = |wasm: &mut WasmReader, section_header: &mut Option<SectionHeader>| {
        handle_section(wasm, section_header, SectionTy::Custom, |wasm, h| {
            wasm.skip(h.contents.len())
        })
    };

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let types = handle_section(&mut wasm, &mut header, SectionTy::Type, |wasm, _| {
        wasm.read_vec(FuncType::read)
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let imports = handle_section(&mut wasm, &mut header, SectionTy::Import, |wasm, _| {
        wasm.read_vec(Import::read)
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let functions = handle_section(&mut wasm, &mut header, SectionTy::Function, |wasm, _| {
        wasm.read_vec(|wasm| wasm.read_var_u32().map(|u| u as usize))
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let tables = handle_section(&mut wasm, &mut header, SectionTy::Table, |wasm, _| {
        wasm.read_vec(TableType::read)
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let memories = handle_section(&mut wasm, &mut header, SectionTy::Memory, |wasm, _| {
        wasm.read_vec(MemType::read)
    })?
    .unwrap_or_default();
    if memories.len() > 1 {
        return Err(Error::MoreThanOneMemory);
    }

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let globals = handle_section(&mut wasm, &mut header, SectionTy::Global, |wasm, h| {
        globals::validate_global_section(wasm, h)
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let exports = handle_section(&mut wasm, &mut header, SectionTy::Export, |wasm, _| {
        wasm.read_vec(Export::read)
    })?
    .unwrap_or_default();

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let start = handle_section(&mut wasm, &mut header, SectionTy::Start, |wasm, _| {
        wasm.read_var_u32().map(|idx| idx as FuncIdx)
    })?;

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let _: Option<()> = handle_section(&mut wasm, &mut header, SectionTy::Element, |wasm, _| {
        let mut elem_vec: Vec<()> = Vec::new();

         // TODO: replace with wasm.read_vec in the future
         let vec_length = wasm.read_var_u32().unwrap();
         trace!("Element sections no.: {}", vec_length);
         for i in 0..vec_length {
            let ttype = wasm.read_var_u32().unwrap();
            // https://webassembly.github.io/spec/core/binary/modules.html#element-section
            match ttype {
                0 => {
                    // type funcref
                }
                1 => {

                }
                2 => {

                }
                3 => {

                }
                4 => {

                }
                5 => {

                }
                6 => {

                }
                7 => {

                }
                _ => unimplemented!()
            }
         }
        todo!("element section not yet supported")
    })?;

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    // let data_count = handle_section(&mut wasm, &mut header, SectionTy::DataCount, |wasm, _| {
    //     // let size = wasm.read_var_u32().unwrap();
    //     // trace!("Data Count Section Size: {}", size);
    //     // assert_eq!(size, 1);
    //     let len = wasm.read_var_u32().unwrap();
    //     trace!("Data Count: {}", len);
    //     Ok(len)
    // })?
    // .unwrap_or_default();

    // let data_count = {
    //     let section_id = wasm.read_u8().unwrap();
    //     trace!("Data count section id: {}", section_id);
    //     let section_size = wasm.read_var_u32().unwrap();
    //     // assert_eq!(section_size, 2);
    //     let size = wasm.read_var_u32().unwrap();
    //     trace!("Data count: {}", size);
    //     size
    // };

    let _: Option<()> = handle_section(&mut wasm, &mut header, SectionTy::DataCount, |wasm, _| {
        todo!("data count section not yet implemented")

    })?;

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let func_blocks = handle_section(&mut wasm, &mut header, SectionTy::Code, |wasm, h| {
        code::validate_code_section(wasm, h, &types, &functions, &globals)
    })?
    .unwrap_or_default();

    assert_eq!(func_blocks.len(), functions.len(), "these should be equal"); // TODO check if this is in the spec

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

    let data_section = handle_section(&mut wasm, &mut header, SectionTy::Data, |wasm, _| {
        let mut data_vec: Vec<DataType> = Vec::new();

        // TODO: replace with wasm.read_vec in the future
        let vec_length = wasm.read_var_u32().unwrap();
        trace!("Data sections no.: {}", vec_length);
        for i in 0..vec_length {
            let mode = wasm.read_var_u32().unwrap();
            let data_sec: DataType;
            match mode {
                0 => {
                    // active { memory 0, offset e }
                    trace!("Data section #{} is active", i);
                    let offset = {
                        // TODO: actually verify this offset
                        let mut bytes: Vec<u8> = Vec::new();
                        bytes.push(wasm.read_u8().unwrap());
                        while *bytes.last().unwrap() != opcode::END {
                            bytes.push(wasm.read_u8().unwrap());
                        }
                        bytes
                    };
                    data_sec = DataType::ActiveDataForMemoryX(ActiveDataForMemoryX {
                        memory_idx: 0,
                        offset,
                        init: wasm.read_vec(|el| Ok(el.read_u8().unwrap())).unwrap(),
                    });
                }
                1 => {
                    // passive
                    trace!("Data section #{} is active", i);
                    data_sec = DataType::PassiveData(PassiveData {
                        init: wasm.read_vec(|el| Ok(el.read_u8().unwrap())).unwrap(),
                    });
                }
                2 => {
                    // mode active { memory x, offset e }
                    // this hasn't been yet implemented in wasm
                    // as per docs:

                    // https://webassembly.github.io/spec/core/binary/modules.html#data-section
                    // The initial integer can be interpreted as a bitfield. Bit 0 indicates a passive segment, bit 1 indicates the presence of an explicit memory index for an active segment.
                    // In the current version of WebAssembly, at most one memory may be defined or imported in a single module, so all valid active data segments have a memory value of 0
                    unimplemented!();
                }
                _ => unreachable!(),
            };
            trace!("{:#?}", data_sec);
            data_vec.push(data_sec);
        }

        Ok(data_vec)
    })?
    .unwrap_or_default();

    // assert_eq!(data_section.len(), data_count as usize);

    while (skip_section(&mut wasm, &mut header)?).is_some() {}

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
