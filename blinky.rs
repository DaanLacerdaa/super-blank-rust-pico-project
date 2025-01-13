#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _; // Panic handler
use rp2040_hal::{
    clocks::init_clocks_and_plls,
    entry,
    gpio::{Pin, Pins, PushPullOutput},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    Timer,
};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;

// Constantes de tempo em milissegundos
const PONTO: u16 = 200;
const TRACO: u16 = 800;
const TEMPO_GAP: u16 = 125;
const INTERVALO: u16 = 250;
const CICLO: u16 = 3000;

// Função para definir a cor do LED RGB
fn set_color(
    red: &mut Pin<PushPullOutput>,
    green: &mut Pin<PushPullOutput>,
    blue: &mut Pin<PushPullOutput>,
    red_on: bool,
    green_on: bool,
    blue_on: bool,
) {
    red.set_state(red_on.into());
    green.set_state(green_on.into());
    blue.set_state(blue_on.into());
}

// Função para sinalizar com LEDs e buzzers
fn sinalizar(
    red: &mut Pin<PushPullOutput>,
    green: &mut Pin<PushPullOutput>,
    blue: &mut Pin<PushPullOutput>,
    buzzer1: &mut Pin<PushPullOutput>,
    buzzer2: &mut Pin<PushPullOutput>,
    delay: &mut Delay,
    duration: u16,
    use_red: bool,
    use_green: bool,
    use_blue: bool,
) {
    // Define a cor do LED RGB
    set_color(red, green, blue, use_red, use_green, use_blue);

    // Liga os buzzers
    buzzer1.set_high().unwrap();
    buzzer2.set_high().unwrap();
    delay.delay_ms(duration as u32);

    // Desliga LEDs e buzzers
    set_color(red, green, blue, false, false, false);
    buzzer1.set_low().unwrap();
    buzzer2.set_low().unwrap();

    // Pausa entre sinais dentro de uma letra
    delay.delay_ms(TEMPO_GAP as u32);
}

// Função para enviar o sinal SOS
fn envia_sos(
    red: &mut Pin<PushPullOutput>,
    green: &mut Pin<PushPullOutput>,
    blue: &mut Pin<PushPullOutput>,
    buzzer1: &mut Pin<PushPullOutput>,
    buzzer2: &mut Pin<PushPullOutput>,
    delay: &mut Delay,
) {
    // Sinaliza 3 pontos (S) com vermelho
    for _ in 0..3 {
        sinalizar(red, green, blue, buzzer1, buzzer2, delay, PONTO, true, false, false);
    }

    // Pausa entre letras
    delay.delay_ms(INTERVALO as u32);

    // Sinaliza 3 traços (O) com verde
    for _ in 0..3 {
        sinalizar(red, green, blue, buzzer1, buzzer2, delay, TRACO, false, true, false);
    }

    // Pausa entre letras
    delay.delay_ms(INTERVALO as u32);

    // Sinaliza 3 pontos (S) com vermelho
    for _ in 0..3 {
        sinalizar(red, green, blue, buzzer1, buzzer2, delay, PONTO, true, false, false);
    }

    // Pausa antes de reiniciar o ciclo
    delay.delay_ms(CICLO as u32);
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Inicialização dos clocks e watchdog
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp2040_hal::rosc::RingOscillator::new(pac.ROSC).initialize(),
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // Configuração dos GPIOs
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let mut red = pins.gpio13.into_push_pull_output();
    let mut green = pins.gpio11.into_push_pull_output();
    let mut blue = pins.gpio12.into_push_pull_output();
    let mut buzzer1 = pins.gpio10.into_push_pull_output();
    let mut buzzer2 = pins.gpio21.into_push_pull_output();

    // Inicialização do timer e delay
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().0);

    // Loop principal
    loop {
        envia_sos(&mut red, &mut green, &mut blue, &mut buzzer1, &mut buzzer2, &mut delay);
    }
}
