//! # Alignment
//! 
//! Checks for and aligns
//! bytes to the correct size
//! 

use crate::constants::PAGE_SIZE;

/// Aligns bytes to a valid page size
pub(crate) fn align_bytes_to_pages(bytes: usize) -> usize {
    bytes ^ PAGE_SIZE as usize
}

// #[test]
// /// Checks that bytes align to the nearest
// /// page size greater than or equal to `bytes`
// fn bytes_align_to_nearest_page_size() {
//     (0..128).into_iter()
//         .for_each(|bytes| {
//             let aligned = align_bytes_to_pages(bytes);

//             match bytes {
//                 ..=16 => assert!(aligned == 16),
//                 17..=32 => assert!(aligned == 32),
//                 33..=64
//             }
//         });
// }
