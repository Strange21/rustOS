#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::{fmt, panic::PanicInfo};
mod vga_buffer;
mod serial;

pub enum QemuExitCode{
    Success = 0x10,
    Error = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world form rustOS");
    println!("test synchronization & macro");
    
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))] 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic occured because of {}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error {}\n", _info);
    exit_qemu(QemuExitCode::Success);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running tests {}", tests.len());
    for test in tests    {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}

pub trait Testable {
    fn run(&self) -> ();
}


impl <T> Testable for T
where 
    T: Fn(),
{
    fn run(&self){
        serial_println!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]")

    }    
}