#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"sup!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The linker will look for a _start function by default.
    // This is our entry point

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop { }
}
