#![no_std]
#![no_main]

const GPIO_BASE: usize   = 0x3F200000;
const UART0_BASE: usize  = 0x3F201000;

const GPFSEL1: *mut u32  = (GPIO_BASE + 0x04) as *mut u32;
const GPPUD: *mut u32    = (GPIO_BASE + 0x94) as *mut u32;
const GPPUDCLK0: *mut u32 = (GPIO_BASE + 0x98) as *mut u32;

const UART0_DR: *mut u32     = (UART0_BASE + 0x00) as *mut u32;
// const UART0_FR: *const u32   = (UART0_BASE + 0x18) as *const u32;
const UART0_IBRD: *mut u32   = (UART0_BASE + 0x24) as *mut u32;
const UART0_FBRD: *mut u32   = (UART0_BASE + 0x28) as *mut u32;
const UART0_LCRH: *mut u32   = (UART0_BASE + 0x2C) as *mut u32;
const UART0_CR: *mut u32     = (UART0_BASE + 0x30) as *mut u32;
const UART0_IMSC: *mut u32   = (UART0_BASE + 0x38) as *mut u32;

use core::arch::asm;
use core::panic::PanicInfo;



extern "C" {
    static _stack_start_0: u8;
    static _stack_start_1: u8;
    static _stack_start_2: u8;
    static _stack_start_3: u8;
}

extern "C" {
    static __core1_main_start: u8;
    static __core2_main_start: u8;
    static __core3_main_start: u8;
}

mod boot {
    use core::arch::global_asm;
    global_asm!(
        "
            .section .text._start
            .globl _start
        _start:
            ldr x0, = _stack_start_0
            mov sp, x0
            bl _rust_main

            "  
    );
}

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[export_name = "_rust_main"]
pub extern "C" fn rust_main() {
    unsafe {
        zero_bss();

        // 1. Set GPIO14 and GPIO15 to ALT0 (UART)
        let mut ra = core::ptr::read_volatile(GPFSEL1);
        ra &= !((7 << 12) | (7 << 15));       // clear bits for GPIO14, GPIO15
        ra |= (4 << 12) | (4 << 15);          // set ALT0 (100) for GPIO14, GPIO15
        core::ptr::write_volatile(GPFSEL1, ra);
    
        // 2. Disable pull-up/down for pins 14 and 15
        core::ptr::write_volatile(GPPUD, 0);
        for _ in 0..150 { asm!("nop"); }
        core::ptr::write_volatile(GPPUDCLK0, (1 << 14) | (1 << 15));
        for _ in 0..150 { asm!("nop"); }
        core::ptr::write_volatile(GPPUDCLK0, 0);
    
        // 3. Disable UART0
        core::ptr::write_volatile(UART0_CR, 0);
    
        // 4. Clear all interrupts
        core::ptr::write_volatile(UART0_IMSC, 0);
    
        // 5. Set integer & fractional baud rate divisor
        // Assuming UARTCLK = 48MHz and baud = 115200
        // IBRD = int(48_000_000 / (16 * 115200)) = 26
        // FBRD = int(((48_000_000 % (16 * 115200)) * 64 + 0.5) / (16 * 115200)) = 3
        core::ptr::write_volatile(UART0_IBRD, 26);
        core::ptr::write_volatile(UART0_FBRD, 3);
    
        // 6. Line control: 8N1, enable FIFO
        core::ptr::write_volatile(UART0_LCRH, (1 << 4) | (3 << 5)); // FIFO enable + 8 bit word
    
        // 7. Enable UART0 TX, RX, and overall UART
        core::ptr::write_volatile(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));

        for _ in 0..200 { asm!("nop"); }
        core::ptr::write_volatile(UART0_DR, b'H' as u32);

        start_cores();

        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop {
            
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);
            
            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);
            
            for _ in 1..1000000 {
                    asm!("nop");
            }
        }

    }

    
}

unsafe fn zero_bss() {
    let mut bss = &raw mut __bss_start as *mut u64;
    let end = &raw const __bss_end as *const u64;
    while (bss as *const u64) < end {
        core::ptr::write_volatile(bss, 0);
        bss = bss.add(1);
    }
}

unsafe fn start_cores() {

    core::ptr::write_volatile(UART0_DR, b'a' as u32);

    // addresses to write fn pointers to.
    // const CORE1_START: *mut u32 = 0x4000009C as *mut u32;
    // const CORE2_START: *mut u32 = 0x400000AC as *mut u32;
    // const CORE3_START: *mut u32 = 0x400000BC as *mut u32;
    
    const CORE1_START: *mut u32 = 0xE0 as *mut u32;
    const CORE2_START: *mut u32 = 0xE8 as *mut u32;
    const CORE3_START: *mut u32 = 0xF0 as *mut u32;

    core::ptr::write_volatile(UART0_DR, b'b' as u32);

    // write function ptrs to addresses:
    // core::ptr::write_volatile(CORE1_START, core1_main as u32);
    // core::ptr::write_volatile(CORE2_START, core2_main as u32);
    // core::ptr::write_volatile(CORE3_START, core3_main as u32);
    // 
    core::ptr::write_volatile(CORE1_START, 0);
    core::ptr::write_volatile(CORE2_START, 0);
    core::ptr::write_volatile(CORE3_START, 0);

    core::ptr::write_volatile(CORE1_START, 0x90000);
    core::ptr::write_volatile(CORE2_START, 0xA0000);
    core::ptr::write_volatile(CORE3_START, 0xB0000);
    
    core::ptr::write_volatile(UART0_DR, b'c' as u32);
    core::arch::asm!("sev");
    core::ptr::write_volatile(UART0_DR, b'c' as u32);
}

#[no_mangle]
#[link_section = ".core1_main"]
pub extern "C" fn core1_main() -> ! {
    unsafe {

        for _ in 1..(4670000) {
                asm!("nop");
        }

        core::ptr::write_volatile(UART0_DR, b'A' as u32);

        loop {
            for _ in 1..1010000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'1' as u32);

            
        }
    }
}

#[no_mangle]
#[link_section = ".core2_main"]
pub extern "C" fn core2_main() -> ! {
    unsafe {


        for _ in 1..(4500000) {
                asm!("nop");
        }

        core::ptr::write_volatile(UART0_DR, b'B' as u32);

        loop {
            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'2' as u32);

            
        }
    }
}

#[no_mangle]
#[link_section = ".core3_main"]
pub extern "C" fn core3_main() -> ! {
    unsafe {


        for _ in 1..(6780000) {
                asm!("nop");
        }

        core::ptr::write_volatile(UART0_DR, b'C' as u32);

        loop {
            for _ in 1..2300700 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'3' as u32);

            
        }
    }
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
