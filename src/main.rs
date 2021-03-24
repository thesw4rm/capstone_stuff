extern crate rppal;

use rppal::uart::{Parity, Uart};

use std::time::Duration;

fn main() {
    let mut uart = Uart::with_path("/dev/ttyAMA0", 9600, Parity::Even, 8, 2).expect("COULD SET UART");
    uart.set_read_mode(1, Duration::default()).expect("COULD SET UART MODE");
    println!("ASD");
}
