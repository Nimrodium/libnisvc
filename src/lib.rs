/*
 *
 * NISVC Architecture Common Library
 *
 * Used for building various tools in the NISVC Toolchain.
 *
 * Includes
 * * ISA Spec & Structures
 * * NISVC-EF operations
 * *
 */
pub type Bin = Vec<u8>;
mod nisvc_ef;
mod symbol_table;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
