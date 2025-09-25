use crate::Bin;
use crate::symbol_table::SymbolTable;
/// Standard Executable Format for the NISVC Architecture.
/// Unlike ELF or PE this file format is relatively simple. and does not support linking.
/// a NISVC-EF file is split into 3 main sections
/// an optional Shebang string may be prepended to the beginning of the file, in which case the loader will remove it before passing it to the runtime.
/// a block enclosed by angle brackets indicate an optional feature
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
    pub fn new(file: &str) -> Result<Self, String> {
        todo!()
    }
}
