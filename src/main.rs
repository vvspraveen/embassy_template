#![no_std]
#![no_main]

use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;
use riscv_hal::device::DeviceResources;

use riscv_rt::entry;
use riscv_semihosting::hprintln;

const DELAY_MS: u32 = 1_000;


#[entry]
fn main() -> ! {
    hprintln!("Starting GPIO example");

    // Take ownership of the device peripherals
    let dp = DeviceResources::take().unwrap();

    // Split GPIO pins
    let pins = dp.pins;

    // Configure Pin 0 as output
    let mut led = pins.{pin_num}.into_output();

    // Set Pin 0 high
    // led.set_high().unwrap();

    // Main loop: Toggle Pin 0 every second
    loop {
        hprintln!("in loop");
        // delay for 1 second
        delay_ms(DELAY_MS, DELAY_MS);

        // toggle pin 0
        match led.toggle() {
            Ok(_) => hprintln!("Toggled pin 0"),
            Err(_) => hprintln!("Failed to toggle pin 0"),
        }
        hprintln!("toggled done");
    }
}

/// Provides a delay of approximately n milliseconds
#[inline(always)]
fn delay_ms(outer: u32, inner: u32) {
    for _ in 0..outer {
        for _ in 0..inner {
            // Compiler barrier to prevent optimization
            core::hint::spin_loop();
        }
    }
}