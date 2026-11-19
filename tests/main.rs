//! # Tests
//! 
//! Tests for `halloc`
//! 

use std::thread;

use halloc::{PAGE_SIZE, api::functions::get_between, free, malloc};

#[test]
fn main() {
    let mut ptr = malloc::<[u8; 4]>();
    let addr1 = ptr.addr();
    println!("ADDR 1: {:?}", addr1);

    *ptr = [255; 4];
    
    println!("HEAP: {:?}", get_between(0, 4));

    let ptr2 = malloc::<u32>();
    let addr2 = ptr2.addr();
    println!("ADDR 2: {:?}", addr2);

    assert!(addr2 == addr1 + PAGE_SIZE, "{} - {} = {}", addr2, addr1, addr2 - addr1);

    free(ptr);

    let handle = thread::spawn(move || {
        let new_ptr = malloc::<[u8; 4]>();
        let new_addr = new_ptr.addr();
        println!("NEW: {:?}, {:?}", new_addr, *new_ptr);
        new_addr
    });

    if let Ok(addr) = handle.join() {
        assert!(addr == addr2 + PAGE_SIZE);
    } else {
        panic!("Handle didn't join");
    }
}
