// Ref: riscv/opensbi/lib/utils/serial/uart8250.c
#![allow(unused)]
use core::fmt::{self, Write};

const VIRT_UART16550_ADDR: *mut u8 = 0x10000000 as *mut _;
const VIRT_UART_IN_FREQ: u32 = 1843200;
const VIRT_UART_BAUDRATE: u32 = 115200;

const UART_RBR_OFFSET: usize = 0; /* In:  Recieve Buffer Register */
const UART_THR_OFFSET: usize = 0; /* Out: Transmitter Holding Register */
const UART_DLL_OFFSET: usize = 0; /* Out: Divisor Latch Low */
const UART_IER_OFFSET: usize = 1; /* I/O: Interrupt Enable Register */
const UART_DLM_OFFSET: usize = 1; /* Out: Divisor Latch High */
const UART_FCR_OFFSET: usize = 2; /* Out: FIFO Control Register */
const UART_IIR_OFFSET: usize = 2; /* I/O: Interrupt Identification Register */
const UART_LCR_OFFSET: usize = 3; /* Out: Line Control Register */
const UART_MCR_OFFSET: usize = 4; /* Out: Modem Control Register */
const UART_LSR_OFFSET: usize = 5; /* In:  Line Status Register */
const UART_MSR_OFFSET: usize = 6; /* In:  Modem Status Register */
const UART_SCR_OFFSET: usize = 7; /* I/O: Scratch Register */
const UART_MDR1_OFFSET: usize = 8; /* I/O:  Mode Register */

const UART_LSR_FIFOE: u8 = 0x80; /* Fifo error */
const UART_LSR_TEMT: u8 = 0x40; /* Transmitter empty */
const UART_LSR_THRE: u8 = 0x20; /* Transmit-hold-register empty */
const UART_LSR_BI: u8 = 0x10; /* Break interrupt indicator */
const UART_LSR_FE: u8 = 0x08; /* Frame error indicator */
const UART_LSR_PE: u8 = 0x04; /* Parity error indicator */
const UART_LSR_OE: u8 = 0x02; /* Overrun error indicator */
const UART_LSR_DR: u8 = 0x01; /* Receiver data ready */
const UART_LSR_BRK_ERROR_BITS: u8 = 0x1E; /* BI, FE, PE, OE bits */

unsafe fn write_reg(index: usize, byte: u8) {
    core::ptr::write_volatile(VIRT_UART16550_ADDR.add(index), byte)
}

unsafe fn read_reg(index: usize) -> u8 {
    core::ptr::read_volatile(VIRT_UART16550_ADDR.add(index))
}

pub fn virt_console_init() {
    let uart8250_in_freq = VIRT_UART_IN_FREQ;
    let uart8250_baudrate = VIRT_UART_BAUDRATE;
    let bdiv = uart8250_in_freq / (16 * uart8250_baudrate);
    unsafe { 
        /* Disable all interrupts */
        write_reg(UART_IER_OFFSET, 0x00);
        /* Enable DLAB */
        write_reg(UART_LCR_OFFSET, 0x80);
        /* Set divisor low byte */
        write_reg(UART_DLL_OFFSET, (bdiv & 0xff) as u8);
        /* Set divisor high byte */
        write_reg(UART_DLM_OFFSET, ((bdiv >> 8) & 0xff) as u8);
        /* 8 bits, no parity, one stop bit */
        write_reg(UART_LCR_OFFSET, 0x03);
        /* Enable FIFO */
        write_reg(UART_FCR_OFFSET, 0x01);
        /* No modem control DTR RTS */
        write_reg(UART_MCR_OFFSET, 0x00);
        /* Clear line status */
        let _ = read_reg(UART_LSR_OFFSET);
        /* Read receive buffer */
        let _ = read_reg(UART_RBR_OFFSET);
        /* Set scratchpad */
        write_reg(UART_SCR_OFFSET, 0x00);
    }
}

pub fn virt_console_putc(byte: u8) {
    unsafe {
        while (read_reg(UART_LSR_OFFSET) & UART_LSR_THRE) == 0 {}
        write_reg(UART_THR_OFFSET, byte);
    }
}

pub fn virt_console_getc() -> Option<u8> {
    unsafe {
        if read_reg(UART_LSR_OFFSET) & UART_LSR_DR != 0 {
            Some(read_reg(UART_RBR_OFFSET))
        } else {
            None
        }
    }
}
