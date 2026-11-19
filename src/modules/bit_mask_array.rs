//! # Bit Mask Array
//! 
//! An array of `u8`
//! where each bit is
//! an index into the array
//! 

use core::ops::Index;

/// An array that uses a single bit to
/// represent an index
pub(crate) struct BitMaskArray<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize, Idx: Into<usize>> Index<Idx> for BitMaskArray<SIZE> {
    type Output = bool;

    fn index(&self, index: Idx) -> &Self::Output {
        let index = index.into();
        let bit = index % 8;
        let index = index / 8;

        if self.0[index] & (1 << bit) == (1 << bit) {
            &true
        } else {
            &false
        }
    }
}

impl<const SIZE: usize> BitMaskArray<SIZE> {
    /// Creates a new `BitMaskArray`
    pub(crate) const fn new() -> Self {
        Self([0; SIZE])
    }

    /// Toggles the value of a bit
    /// at bit-index `index`
    pub(crate) fn toggle(
        &mut self,
        index: impl Into<usize>,
    ) {
        let index = index.into();
        let bit = index % 8;
        let index = index / 8;

        self.0[index] ^= 1 << bit;
    }

    /// Gets the amount of bits
    /// in the array
    pub(crate) fn bits(&self) -> usize {
        SIZE * 8
    }
}

/// Checks to make sure new bitmasks
/// have enough bits to represent `SIZE`
#[test]
fn new_bitmask_has_expected_amount_of_bits() {
    // Multiple of 8
    let bit_mask_array = BitMaskArray::<8>::new();
    assert!(bit_mask_array.bits() == 8 * 8);

    // Non multiple of 8
    let bit_mask_array = BitMaskArray::<15>::new();
    assert!(bit_mask_array.bits() == 15 * 8, "Bits: {}", bit_mask_array.bits());
}

/// Checks that indexing into the bitmask
/// selects the correct bit
#[test]
fn bitmask_indexes_correctly() {
    const SIZE: usize = 1;
    let mut mask = BitMaskArray::<SIZE>::new();
    mask.0[0] ^= 1 << 2;

    assert!(mask.0[0] == 4, "{}", mask.0[0]);
    assert!(mask[2_usize])
}

/// Checks to make sure individual bits
/// read and toggle correctly by index
#[test]
fn bits_read_and_write_correctly() {
    const SIZE: usize = 8;
    let mut bit_mask_array = BitMaskArray::<SIZE>::new();

    // Initial read
    //
    // Everything should come back false
    // as nothing has been written yet
    (0..(SIZE * 8)).into_iter()
        .for_each(|i| {
            assert!(!bit_mask_array[i])
        });

    // Initial write
    //
    // Step by 2 so each even bit should
    // come back as true, and each odd bit
    // should be false when read later
    (0..(SIZE * 8)).into_iter()
        .step_by(2)
        .for_each(|i| {
            bit_mask_array.toggle(i);
        });

    // Second read
    //
    // Every even bit should be true
    // and every odd bit should be false
    (0..(SIZE * 8)).into_iter()
        .for_each(|i| {
            if i % 2 == 0 {
                assert!(bit_mask_array[i])
            } else {
                assert!(!bit_mask_array[i])
            }
        });
}
