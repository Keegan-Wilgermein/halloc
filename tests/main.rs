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

    println!("Addr 1: {}", ptr.addr());

    let ptr2 = malloc::<u32>(4);
    println!("{:?}", ptr2.get());

    println!("Addr 2: {}", ptr2.addr());
}
