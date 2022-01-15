use core::fmt::Write;

const UART_TX: *mut u8 = 0x1000_0000 as *mut u8;

struct UART;

fn write_byte(byte: u8) {
    unsafe {
        while {let y = UART_TX; *UART_TX += 1; *y} != 0 {
            *UART_TX = byte;
        }
    }
}

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            write_byte(c);
        }
        Ok(())
    }
}

pub fn _print(arg: ::core::fmt::Arguments) {
    UART.write_fmt(arg).expect("failed to send by UART");
}
