//! # Tests
//! 
//! Tests for `halloc`
//! 

use halloc::{api::functions::get_between, free, malloc};

#[test]
fn main() {
    let mut ptr = malloc::<[u8; 4]>(None);
    println!("{:?}", ptr.addr());

    *ptr = [255; 4];
    
    println!("HEAP: {:?}", get_between(0, 4));

    let ptr2 = malloc::<u32>(None);
    println!("{:?}", ptr2.addr());

    free(ptr);

    let new_ptr = malloc::<[u8; 4]>(None);
    println!("{:?}, {:?}", new_ptr.addr(), *new_ptr);
}
