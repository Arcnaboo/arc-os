#![no_std]

pub mod vga_buffer;
pub mod ke;
pub mod ob;
pub mod mm;
pub mod io;
pub mod cm;
pub mod lpc;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn KiSystemStartup() -> ! {
    println!("Arc-OS Kernel Executive Initializing...");
    println!("Loading HAL...");
    println!("Starting Scheduler...");
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
