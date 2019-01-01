#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

extern crate bootloader_precompiled;

mod vga_buff;
//use crate::vga_buff::{Writer,ColorCode,Color,Buffer};




#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {

    let mut w = vga_buff::Writer::new();
    w.write_byte(b'H');
    w.write_str("Ploopoo");

    loop {}
}
