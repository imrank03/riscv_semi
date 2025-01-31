#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;
use riscv_semihosting::{hprintln, debug};
// use semihosting::println;

#[entry]
fn main() -> ! {

    hprintln!("Hello, world!");
    debug::exit(debug::EXIT_FAILURE);
    loop {
        // Main loop
    }
}