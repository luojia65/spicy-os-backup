use riscv::register::*;

pub fn init() {
    println!("++++ setup interrupt! ++++");
    unsafe {
        mscratch::write(0);
        mtvec::write(trap_handler as usize, mtvec::TrapMode::Direct);
    }
    println!("++++ setup interrupt! ++++");
}

fn trap_handler() -> ! {
    let cause = mcause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
    panic!("trap handled!");
}
