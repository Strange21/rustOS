#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]


use core::panic::PanicInfo;
mod vga_buffer;

static HELLO:&[u8] = "Hello World from rustOS".as_bytes();


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    // for (i , v) in HELLO.iter().enumerate(){
    //     unsafe {
    //         *vga_buf.offset(i as isize * 2) = *v;
    //         *vga_buf.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}