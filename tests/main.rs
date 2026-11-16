//! # Tests
//! 
//! Tests for `halloc`
//! 

use halloc::{PAGE_SIZE, RawPtrOps, malloc, };

#[test]
fn main() {
    let ptr = malloc::<[u8; 4]>(None);
    println!("{:?}", ptr.get());

    ptr.set([255, 255, 255, 255]);
    println!("{:?}", ptr.get());

    println!("Addr 1: {}", ptr.addr());

    let ptr2 = malloc::<u32>(None);
    println!("{:?}", ptr2.get());

    println!("Addr 2: {}", ptr2.addr());

    assert!(ptr2.addr() - ptr.addr() == PAGE_SIZE)
}
