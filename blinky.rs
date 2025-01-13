#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

use rp2040_hal as hal;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{bank0::Gpio25, PushPullOutput},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use cortex_m_rt::entry;
use embedded_time::rate::*;

#[entry]
fn main() -> ! {
    // Obtenha os periféricos
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Frequência do cristal externo
    let external_xtal_freq_hz = 12_000_000u32;

    // Configure os clocks
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Configuração do pino GPIO
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio25.into_push_pull_output();

    // Configure o delay usando o clock
    let system_clock_hz = clocks.system_clock.freq().to_Hz();
    let mut delay = cortex_m::delay::Delay::new(
        cortex_m::Peripherals::take().unwrap().SYST,
        system_clock_hz,
    );

    loop {
        // Acenda o LED
        led.set_high().unwrap();
        delay.delay_ms(500);

        // Apague o LED
        led.set_low().unwrap();
        delay.delay_ms(500);
    }
}
