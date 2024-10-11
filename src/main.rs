// src/main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start_kernel() -> ! {
    println!("{}", 42);
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kfs::test_panic_handler(info)
}
