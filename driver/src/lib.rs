#![no_std]

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0 /* NT_STATUS_SUCCESS */
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
