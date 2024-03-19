// advanced features example code

// "Unsafe rust"
pub fn raw_pointers() {
    // create pointer to arbitrary memory address (bad decision)
    let address = 0x012345usize;
    let r_pointer = address as * const i32;
    // necessarily
    unsafe {
        println!("r_pointer contain: {}", *r_pointer);
    }
}