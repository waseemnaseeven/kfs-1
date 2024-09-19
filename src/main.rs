#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Writing directly in VGA buffer "Hello, World!"
// 'b' prefix = byte string, every character mean an ASCII character
static HELLO: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 0xb8000 : adress of VGA buffer, espace mem dedie au display text sur system x_86
    // *mut u8 : mutable pointer to an byte (octet)
    let vga_buffer = 0xb8000 as *mut u8;

    // enumerate() give index i and a reference to byte for every character
    for (i, &byte) in HELLO.iter().enumerate() {
        // unsecure access to the memory
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// [unstable]
// build-std-features = ["compiler-builtins-mem"]
// build-std = ["core", "compiler_builtins"]

// [build]
// target = "x86_64-kfs-1.json"
