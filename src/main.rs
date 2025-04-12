
#![no_std]
#![no_main]


const GPIO_BASE: usize   = 0x3F200000;
const UART0_BASE: usize  = 0x3F201000;

const GPFSEL1: *mut u32  = (GPIO_BASE + 0x04) as *mut u32;
const GPPUD: *mut u32    = (GPIO_BASE + 0x94) as *mut u32;
const GPPUDCLK0: *mut u32 = (GPIO_BASE + 0x98) as *mut u32;

const UART0_DR: *mut u32     = (UART0_BASE + 0x00) as *mut u32;
const UART0_FR: *const u32   = (UART0_BASE + 0x18) as *const u32;
const UART0_IBRD: *mut u32   = (UART0_BASE + 0x24) as *mut u32;
const UART0_FBRD: *mut u32   = (UART0_BASE + 0x28) as *mut u32;
const UART0_LCRH: *mut u32   = (UART0_BASE + 0x2C) as *mut u32;
const UART0_CR: *mut u32     = (UART0_BASE + 0x30) as *mut u32;
const UART0_IMSC: *mut u32   = (UART0_BASE + 0x38) as *mut u32;

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;
    global_asm!(
        "
        .section .text._start
        .globl _start
        "
    );
}



#[no_mangle]
pub extern "C" fn _start() -> ! {

    const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;

    unsafe {
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
