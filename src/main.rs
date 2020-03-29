#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

#[macro_use]
mod io;
mod sbi;
mod platform;
mod context;
mod trap;

use platform::qemu_virt::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    power_down()
}

#[no_mangle]
extern "C" fn abort() -> ! {
    power_down()
}

global_asm!(include_str!("boot/entry32.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    crate::trap::init();
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("hello world!");
    unsafe {
        asm!("ebreak"::::"volatile");
    }
    power_down();
}
