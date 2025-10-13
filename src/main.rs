#![no_std]
#![no_main]
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal::digital::OutputPin;
use riscv::register::{mie, mstatus};
use riscv_hal::{device::DeviceResources, serial::Serial, time::Bps};
#[embassy_executor::task]
async fn uart_task(serial: riscv_hal::serial::Serial<riscv_pac::Uart0>) {
    loop {
        let _ = serial.write_str("Hello from UART task!\n");
        Timer::after(Duration::from_millis(1)).await;
    }
}
#[embassy_executor::task]
async fn blink_led(
    mut led: riscv_hal::gpio::gpio::Pin22<riscv_hal::gpio::Output<riscv_hal::gpio::Unknown>>,
) {
    loop {
        led.set_high().unwrap();
        Timer::after(Duration::from_millis(1)).await;
        led.set_low().unwrap();
        Timer::after(Duration::from_millis(1)).await;
    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    riscv_hal::time_driver::init();
    unsafe {
        mie::set_mtimer();
        mstatus::set_mie();
    }
    let dp = DeviceResources::take().unwrap();
    let pins = dp.pins;
    let led = pins.pin22.into_output();
    let uart0 = unsafe { riscv_pac::Uart0::steal() };
    let serial = Serial::new(uart0, Bps(115200));
    spawner.must_spawn(blink_led(led));
    spawner.must_spawn(uart_task(serial));
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
