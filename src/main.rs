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
    virt_console_putc(b'\n');
    virt_console_putc(b'P');
    virt_console_putc(b'a');
    virt_console_putc(b'n');
    virt_console_putc(b'i');
    virt_console_putc(b'c');
    virt_console_putc(b'\n');
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
    puts("1235");
    virt_console_putc(b'B');
    virt_console_putc(b'\n');
    puts("1235");
    println!();
    virt_console_putc(b'6');
    virt_console_putc(b'\n');
    // crate::trap::init();
    // unsafe {
    //     asm!("ebreak"::::"volatile");
    // }
    power_down();
}
