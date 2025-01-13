#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use rp2040_hal::{
    pac,
    sio::Sio,
    gpio::{Pin, Output, PushPull},
    timer::Timer,
    watchdog::Watchdog,
    clocks::init_clocks_and_plls,
};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _; // Panic handler

#[rp2040_hal::entry]
fn main() -> ! {
    // Inicialização do hardware
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Configuração do watchdog e clocks
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp2040_hal::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Inicializa o Timer
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Configuração dos pinos de LEDs e buzzer
    let sio = Sio::new(pac.SIO);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_red = pins.gpio13.into_push_pull_output();
    let mut led_green = pins.gpio14.into_push_pull_output();
    let mut led_blue = pins.gpio15.into_push_pull_output();
    let mut buzzer_1 = pins.gpio16.into_push_pull_output();
    let mut buzzer_2 = pins.gpio17.into_push_pull_output();

    // Inicializa LEDs e buzzer
    led_red.set_low().unwrap();
    led_green.set_low().unwrap();
    led_blue.set_low().unwrap();
    buzzer_1.set_low().unwrap();
    buzzer_2.set_low().unwrap();

    // Loop principal
    loop {
        // Alternância simples entre LEDs
        led_red.set_high().unwrap();
        led_green.set_low().unwrap();
        led_blue.set_low().unwrap();
        delay.delay_ms(500);

        led_red.set_low().unwrap();
        led_green.set_high().unwrap();
        led_blue.set_low().unwrap();
        delay.delay_ms(500);

        led_red.set_low().unwrap();
        led_green.set_low().unwrap();
        led_blue.set_high().unwrap();
        delay.delay_ms(500);

        // Controle simples dos buzzers
        buzzer_1.set_high().unwrap();
        buzzer_2.set_low().unwrap();
        delay.delay_ms(1000);

        buzzer_1.set_low().unwrap();
        buzzer_2.set_high().unwrap();
        delay.delay_ms(1000);
    }
}