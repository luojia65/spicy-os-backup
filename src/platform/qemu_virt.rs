const VIRT_TEST_ADDR: *mut u16 = 0x100000 as *mut _;
const VIRT_TEST_FINISHER_PASS: u16 = 0x5555;

pub fn power_down() -> ! {
    unsafe {
        core::ptr::write_volatile(VIRT_TEST_ADDR, VIRT_TEST_FINISHER_PASS);
        core::hint::unreachable_unchecked()
    }
}
