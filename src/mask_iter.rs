use std::convert::From;
use std::ops::ShlAssign;

/// This supposedly dead-simple iterator cycles through all the single-bit bitmasks 
/// in an unsigned word, starting with the LSB.
/// In other words, 1,2,4,8,16,...

// It is part of an experiment to see just how optimized rust code is after inlining.
// More to come.

// [WHINE] This does not compile.
//type UINT = Copy + PartialOrd + From<u8> + ShlAssign<i32>;

// [WHINE] Because rust doesn't do duck-typing, this is not useful, at least not without client pain.
// trait Uint: Copy + PartialOrd + From<u8> + ShlAssign<i32> {}

pub struct MaskIter<T> {
    mask: T, // current mask value
}

// Only "From<u8>" is needed as a trait bound in this fn; the rest are copied from the Iterator impl
// so that a wrong type error shows up in new(), not later.
// [WHINE] Too bad there's no clean way to have DRY trait bounds here; see attempts above.
impl<T: Copy + PartialOrd + From<u8> + ShlAssign<i32>> MaskIter<T> {
    pub fn new() -> MaskIter<T> {
        // [WHINE] "1 as T" doesn't compile, because there's no way (AFAIK) to tell the compiler
        // "T is a number" that will allow this.  from(1) inlines into "1 as T".
        MaskIter { mask: T::from(1) }
    }
}
impl<T: Copy + PartialOrd + From<u8> + ShlAssign<i32>> Iterator for MaskIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // [WHINE] can't match on T::from(0) as in "T::from(0) => None,"
        if self.mask == T::from(0) {
            None
        } else {
            let ret = Some(self.mask);
            self.mask <<= 1;
            ret
        }
    }
}

#[test]
fn test_mask_iter() {
    let m = MaskIter::new();
    let bits: Vec<u32> = m.collect();
    assert_eq!(bits.len(), 32);
    assert_eq!(bits[0], 1u32);
    assert_eq!(bits[31], 1u32 << 31);
}
