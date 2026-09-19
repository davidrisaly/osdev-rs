#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]

mod header;
use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga: *mut u16 = 0xb8000 as *mut u16;

    for i in 0..=(80 * 25) {
        unsafe { *vga.wrapping_add(i) = 0x0000 };
    }

    let msg = "Hello world!";
    for (i, c) in msg.chars().enumerate() {
        unsafe { *vga.wrapping_add(i) = 0x0f00 | c as u16 };
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
