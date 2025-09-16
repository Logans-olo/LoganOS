
#![no_std]
#![no_main]
mod vga_buffer;
use::core::panic::PanicInfo;
static LOGAN: &[u8] = b"My name is Logan";

#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! { 
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in LOGAN.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
