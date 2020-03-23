mod uart;
mod test;
pub use uart::*;
pub use test::*;

use core::fmt::{self, Write};

struct Stdout(());

static STDOUT: spin::Mutex<Stdout> = spin::Mutex::new(Stdout(()));

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            uart::virt_console_putc(ch as u8);
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
