#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn KiSystemStartup() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
