//! # Tests
//! 
//! Tests for `halloc`
//! 

use halloc::{RawPtrOps, malloc};

#[test]
fn main() {
    let ptr = malloc::<[u8; 4]>(4);
    println!("{:?}", ptr.get());

    ptr.set([255, 255, 255, 255]);
    println!("{:?}", ptr.get());

    let ptr2 = malloc::<u32>(4);
    println!("{:?}", ptr2.get());
}
