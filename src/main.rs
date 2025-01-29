#![no_std]
#![no_main]

// use panic_halt as _;
use riscv_rt::entry;
// use riscv_semihosting::hprintln;
use semihosting::println;

#[entry]
fn main() -> ! {

    println!("Hello, world!");

    loop {
        // Main loop
    }
}