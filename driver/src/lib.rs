#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0 /* STATUS_SUCCESS */
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
