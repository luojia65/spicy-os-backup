#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

#[macro_use]
mod platform;
// mod context;
// mod trap;

use platform::qemu_virt::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    virt_console_putc(b'P');
    // println!("{}", info);
    power_down()
}

#[no_mangle]
extern "C" fn abort() -> ! {
    virt_console_putc(b'A');
    power_down()
    // panic!("abort!");
}

global_asm!(include_str!("boot/entry32.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    virt_console_init();
    // for ch in [b'1', b'2', b'3'].iter() {
    //     virt_console_putchar(*ch);
    // }
    virt_console_putc(b'N');
    virt_console_putc(b'B');
    virt_console_putc(b'\n');
    // puts("123");
    println!();
    virt_console_putc(b'6');
    // crate::trap::init();
    // unsafe {
    //     asm!("ebreak"::::"volatile");
    // }
    power_down();
}
