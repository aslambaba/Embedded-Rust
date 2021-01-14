#![no_main]
#![no_std]

use::{entry}

![entry]
fn main() {

    // There We Will use Raw pointer of GPIOE with unsafe rust

    unsafe{

        const GPIOE_BSRR:u32 = 0x48001018;

        // Use ptr::write_volatile for handle the misOptimization during --release build

        ptr::write_volatile((GPIOE_BSRR as *mut u32,1<<9);
        ptr::write_volatile((GPIOE_BSRR as *mut u32,1<<11);
        ptr::write_volatile((GPIOE_BSRR as *mut u32,1<<(9+16));
        ptr::write_volatile((GPIOE_BSRR as *mut u32,1<<(11+16));

    }
    loop{}
}
