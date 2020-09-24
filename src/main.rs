#![no_std]
#![no_main]

mod vga_driver;
extern crate rlibc;
use core::panic::PanicInfo;

#[no_mangle]
fn _start() {
    vga_driver::writer("jml")
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
