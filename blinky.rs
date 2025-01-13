#![no_std]
#![no_main]

use panic_halt as _;
use rp2040_hal as hal;
use hal::{pac, sio::Sio, gpio::{Pin, Output, PushPull}, timer::Delay};
use embedded_time::duration::Milliseconds;

const LED_R_PIN: u8 = 13; // GPIO do LED vermelho
const LED_G_PIN: u8 = 11; // GPIO do LED verde
const LED_B_PIN: u8 = 12; // GPIO do LED azul
const BUZZER_PIN_1: u8 = 10; // GPIO do buzzer 1
const BUZZER_PIN_2: u8 = 21; // GPIO do buzzer 2

const PONTO: u32 = 200; // Duração do ponto
const TRACO: u32 = 800; // Duração do traço
const TEMPO_GAP: u32 = 125; // Gap entre os sinais
const INTERVALO: u32 = 250; // Intervalo entre os grupos de letras
const CICLO: u32 = 3000; // Intervalo total do ciclo SOS

#[rp2040_hal::entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0);
    let delay = Delay::new(pac.TIMER, pac.RESETS);

    let mut led_red = pins.gpio13.into_push_pull_output();
    let mut led_green = pins.gpio11.into_push_pull_output();
    let mut led_blue = pins.gpio12.into_push_pull_output();
    let mut buzzer_1 = pins.gpio10.into_push_pull_output();
    let mut buzzer_2 = pins.gpio21.into_push_pull_output();

    loop {
        envia_sos(
            &mut led_red,
            &mut led_green,
            &mut led_blue,
            &mut buzzer_1,
            &mut buzzer_2,
            &delay,
        );
    }
}

fn set_color(
    red_on: bool,
    green_on: bool,
    blue_on: bool,
    led_red: &mut Pin<Output>,
    led_green: &mut Pin<Output>,
    led_blue: &mut Pin<Output>,
) {
    led_red.set_state(red_on);
    led_green.set_state(green_on);
    led_blue.set_state(blue_on);
}

fn sinalizar(
    duration: u32,
    use_red: bool,
    use_green: bool,
    use_blue: bool,
    led_red: &mut Pin<Output>,
    led_green: &mut Pin<Output>,
    led_blue: &mut Pin<Output>,
    buzzer_1: &mut Pin<Output>,
    buzzer_2: &mut Pin<Output>,
    delay: &Delay,
) {
    set_color(use_red, use_green, use_blue, led_red, led_green, led_blue);

    buzzer_1.set_high();
    buzzer_2.set_high();
    delay.delay_ms(Milliseconds(duration as u32));

    set_color(false, false, false, led_red, led_green, led_blue);
    buzzer_1.set_low();
    buzzer_2.set_low();

    delay.delay_ms(Milliseconds(TEMPO_GAP as u32));
}

fn envia_sos(
    led_red: &mut Pin<Output>,
    led_green: &mut Pin<Output>,
    led_blue: &mut Pin<Output>,
    buzzer_1: &mut Pin<Output>,
    buzzer_2: &mut Pin<Output>,
    delay: &Delay,
) {
    // Envia 3 pontos (S) com vermelho
    for _ in 0..3 {
        sinalizar(PONTO, true, false, false, led_red, led_green, led_blue, buzzer_1, buzzer_2, delay);
    }

    delay.delay_ms(Milliseconds(INTERVALO as u32));

    // Envia 3 traços (O) com verde
    for _ in 0..3 {
        sinalizar(TRACO, false, true, false, led_red, led_green, led_blue, buzzer_1, buzzer_2, delay);
    }

    delay.delay_ms(Milliseconds(INTERVALO as u32));

    // Envia 3 pontos (S) com vermelho
    for _ in 0..3 {
        sinalizar(PONTO, true, false, false, led_red, led_green, led_blue, buzzer_1, buzzer_2, delay);
    }

    delay.delay_ms(Milliseconds(CICLO as u32));
}
