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
mod abi;
mod constant;
mod nisvc_ef;
mod symbol_table;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// pub fn to_u16(bytes: &[u8]) -> u16 {

// }
// pub fn bytes_to<T>(bytes: &[u8]) -> T
// where
//     T: Sized + Default,
// {
//     if bytes.len() != size_of::<T>() {
//         panic!("cannot cast {:?}", bytes)
//     }
//     let t = T::default();

//     // unsafe { std::mem::transmute(bytes) }
//     todo!()

// }
pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    // let u64size = size_of::<u64>();
    let mut buf: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
    // let raw = bytes.as_ptr();
    // println!("bytes slice size {}", bytes.len());
    if bytes.len() > size_of::<u64>() {
        panic!(
            "attempted to build a u64 composed of more than {} bytes, byte slice: {bytes:?} len: {}",
            size_of::<u64>(),
            bytes.len()
        )
    }
    for (i, b) in bytes.iter().enumerate() {
        // let buf.get(i).expect("attempted to build a u64 composed of more than 8 bytes, panic on access of buf[{i}] which is out of bounds")
        buf[i] = *b;
    }
    // buf.copy_from_slice(bytes);
    u64::from_le_bytes(buf)
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
