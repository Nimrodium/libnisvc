use crate::Bin;
use crate::constant::{MAGIC, SHEBANG};
use crate::symbol_table::SymbolTable;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::Read;
use std::path::Path;
/// Standard Executable Format for the NISVC Architecture.
/// Unlike ELF or PE this file format is relatively simple. and does not support linking.
/// a NISVC-EF file is split into 3 main sections:
/// * an optional Shebang string may be prepended to the beginning of the file, in which case the loader will remove it before passing it to the runtime.
/// > a block enclosed by angle brackets indicate an optional feature
/// <[ (#!/usr/bin/env nisvc-system) shebang ]>
/// [ (NISVC-EF) magic string ]
/// [ nisvc version number ]
/// [ include symbol table ]
/// <[ symbol table ]>
/// [ binary executable ]
/// [ static data ]
pub struct NISVCExecutableFormat {
    symbol_table: Option<SymbolTable>,
    image: Bin,
}
impl NISVCExecutableFormat {
    // loads file
    pub fn new(symbol_table: Option<SymbolTable>, image: Bin) -> Self {
        Self {
            symbol_table,
            image,
        }
    }

    pub fn load(file: &Path) -> Result<Self> {
        let mut bin: Bin = Bin::new();
        File::open(file)
            .map_err(|e| anyhow!("failed to open file: {e}"))?
            .read_to_end(&mut bin)?;
        if bin[0..1] == *b"#!" {
            let shebang_length = bin
                .iter()
                .position(|byte| *byte as char == '\n')
                .unwrap_or(0);
            bin = bin.drain(0..shebang_length).collect();
        }
        todo!()
    }

    /// builds the NISVC-EF into file
    pub fn build(self, include_shebang: bool, nisvc_version: &str) -> Result<Bin> {
        // todo!()
        let mut bin = Bin::new();
        if include_shebang {
            bin.extend(SHEBANG)
        }
        bin.extend(MAGIC);
        bin.extend(nisvc_version.as_bytes());
        if let Some(symbol_table) = &self.symbol_table {
            let symbol_table_image = symbol_table.serialize()?;
            let symbol_table_size: u64 = symbol_table_image.len() as u64;
            bin.extend(1u8.to_le_bytes()); // symbol table present
            bin.extend(symbol_table_size.to_le_bytes());
            bin.extend(symbol_table_image);
        }
        bin.extend(self.image);
        Ok(bin)
    }
}
