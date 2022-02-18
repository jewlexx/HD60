#![no_std]
#![no_main]

use winapi::km::wdm::{DbgPrint, DRIVER_OBJECT};
use winapi::shared::ntdef::UNICODE_STRING;
use winapi::shared::{ntdef::NTSTATUS, ntstatus::STATUS_SUCCESS};

#[no_mangle]
pub extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _: *const UNICODE_STRING,
) -> NTSTATUS {
    unsafe {
        DbgPrint("Hello, world!\0".as_ptr());
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub extern "system" fn driver_exit(driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("Bye bye!\0".as_ptr());
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
