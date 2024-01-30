#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(m_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use m_os::{hlt_loop, println};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // use m_os::memory::active_level_4_table;

    println!("Starting Mos...\n\n");

    m_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    hlt_loop()
}

// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    m_os::test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
