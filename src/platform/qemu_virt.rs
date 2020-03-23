mod uart;
mod test;
pub use uart::*;
pub use test::*;

use core::fmt::{self, Write};

#[doc(hidden)] // todo remove
pub fn puts(s: &str) {
    for ch in s.as_bytes() {
        // panic!("!");
        uart::virt_console_putc(*ch);
    }
}

struct Stdout(());

static STDOUT: spin::Mutex<Stdout> = spin::Mutex::new(Stdout(()));

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &ch in s.as_bytes() {
            uart::virt_console_putc(ch);
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
    // STDOUT is unlocked when dropped
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::platform::qemu_virt::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
