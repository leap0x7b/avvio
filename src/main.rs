#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod uart; // this is required for some dumb reason
use core::arch::asm;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($arg:expr) => (print!(concat!($arg, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
extern "C" fn kmain() {
    let mut con_uart = uart::Uart::new(0x1000_0000);
    con_uart.init();
    print!("Hello, world!");
}
