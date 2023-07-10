// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

pub mod uart;
use uart::Uart;

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    Uart::init("TX", "RX", 115200, |uart, c| {
        if c as char == '\n' {
            let content = uart.read_string(uart.available());
            uart.write_string(format!("You sent: \"{}\"", content.trim_end()));
        }
    });
}
