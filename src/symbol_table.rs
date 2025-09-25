use crate::Bin;
use bidirectional_map::Bimap;
use std::{collections::HashMap, rc::Rc};
/// Table for associating addresses with named symbols for debugging.
///
/// # Serial Format
/// each entry is >9 bytes and is formated as such:
/// ```
///
/// %per symbol%...
/// [ symbol len ]
/// [ symbol str ]
/// [ u64 address ]
/// %end%
/// ```
const EOF_SYMBOLTABLE_ERR: &str = "Unexpected EOF while parsing SymbolTable";
pub struct SymbolTable {
    map: Bimap<Rc<String>, u64>, // Symbol < - > Address
}
impl SymbolTable {
    /// initialize a new table
    pub fn new() -> Self {
        Self { map: Bimap::new() }
    }

    /// load a symbol table from serialized format
    pub fn deserialize(serialized: Bin) -> Result<Self, String> {
        let mut stream = serialized.into_iter();
        loop {
            let symbol = {
                let str_len = stream.next().ok_or(EOF_SYMBOLTABLE_ERR)?;
                let mut strb = Vec::<u8>::new();
                // likely possible to collapse into a chain w. an iterator
                for _ in 0..str_len {
                    strb.push(stream.next().ok_or(EOF_SYMBOLTABLE_ERR)?);
                }
                String::try_from(strb)
                    .map_err(|e| format!("Symbol contained illegal characters {e}"))?
            };
            let address = {
                stream.
            };
        }
        todo!()
    }

    /// serialize SymbolTable
    pub fn serialize(&self) -> Result<Bin, String> {
        todo!()
    }
    /// get a symbol from an associated address
    pub fn get_symbol(&self, address: u64) -> Option<String> {
        todo!()
    }
    /// get an address from an associated symbol
    pub fn get_address(&self, symbol: &str) -> Option<u64> {
        todo!()
    }
    /// insert an address symbol pair
    pub fn insert(&self, symbol: &str, address: &u64) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::SymbolTable;

    fn test_SymbolTable_serial() {
        let table = SymbolTable::new();
        for (symbol, address) in (1..15).into_iter().enumerate() {
            table.insert(format!("test-{address}").as_str(), &(address as u64));
        }
    }
    fn test_SymbolTable_deserial() {}
}
