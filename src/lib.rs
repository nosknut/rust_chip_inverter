// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

pub mod uart;

use uart::Uart;

static mut JSON: Vec<char> = Vec::new();
static mut NUM_CURLY_BRACES: i32 = 0;

pub struct JsonParser {
    // json: String,
    // num_curly_braces: i32,
}

impl JsonParser {
    pub fn new() -> JsonParser {
        JsonParser {
            // json: String::from(""),
            // num_curly_braces: 0,
        }
    }

    pub fn parse_uart(uart: &mut Uart) {
        while uart.available() > 0 {
            if let Some(c) = uart.read_char() {
                if let Some(js) = JsonParser::parse(c) {
                    let json_string: String = js.into_iter().collect();
                    uart.write_string(format!("You sent: \"{}\"", json_string));
                }
            }
        }
    }

    pub fn parse(c: char) -> Option<Vec<char>> {
        unsafe {
            if NUM_CURLY_BRACES < 0 {
                JSON.clear();
                NUM_CURLY_BRACES = 0;
                return None;
            }

            if NUM_CURLY_BRACES == 0 {
                if c != '{' {
                    if !JSON.is_empty() {
                        JSON.clear();
                    }
                    return None;
                }
            }

            if c == '{' {
                NUM_CURLY_BRACES += 1;
            }

            if c == '}' {
                NUM_CURLY_BRACES -= 1;
            }

            JSON.push(c);

            if NUM_CURLY_BRACES == 0 {
                let content = JSON.clone();
                JSON.clear();
                return Some(content);
            }

            None
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    // let mut parser = JsonParser::new();
    Uart::init("TX", "RX", 115200, |uart, _c| {
        JsonParser::parse_uart(uart);
    });
}
