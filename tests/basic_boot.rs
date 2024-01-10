#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(m_os::test_runner)]

use core::panic::PanicInfo;
use m_os::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    m_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
