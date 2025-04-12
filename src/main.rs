
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
    _start:
        ldr sp, =_stack_top  // set stack pointer
        b _rust_main
        "
    );
}



#[no_mangle]
pub extern "C" fn _start() -> ! {

    const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;

    unsafe {
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);


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

        
        loop {
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);
            
            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);
            
            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'H' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'e' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            core::ptr::write_volatile(UART0_DR, b'l' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'l' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b'o' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b',' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            core::ptr::write_volatile(UART0_DR, b' ' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }
            
            core::ptr::write_volatile(UART0_DR, b'W' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }

            
            core::ptr::write_volatile(UART0_DR, b'o' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            
            core::ptr::write_volatile(UART0_DR, b'r' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            
            core::ptr::write_volatile(UART0_DR, b'l' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            
            core::ptr::write_volatile(UART0_DR, b'd' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            
            core::ptr::write_volatile(UART0_DR, b'!' as u32);

            for _ in 1..1000000 {
                    asm!("nop");
            }


            
            core::ptr::write_volatile(UART0_DR, b'\n' as u32);

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
