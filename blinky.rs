#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use rp2040_hal::{
    pac,
    sio::Sio,
    gpio::{Pin, Output, PushPull},
    timer::Timer,
    watchdog::Watchdog,
};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _; // Panic handler

#[rtic::app(device = rp2040_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_red: Pin<Output<PushPull>>,
        led_green: Pin<Output<PushPull>>,
        led_blue: Pin<Output<PushPull>>,
        buzzer_1: Pin<Output<PushPull>>,
        buzzer_2: Pin<Output<PushPull>>,
        delay: Delay,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // Inicialização do hardware
        let mut pac = ctx.device;
        let mut sio = Sio::new(pac.SIO);
        let mut watchdog = Watchdog::new(pac.WATCHDOG);

        // Inicializa o Timer
        let timer = Timer::new(pac.TIMER);
        let delay = Delay::new(ctx.core.SYST, &mut pac.CLK_SYS);

        // Configuração dos pinos de LEDs e buzzer
        let gpio = pac.GPIO;
        let mut led_red = gpio.get_pin(13).into_push_pull_output();
        let mut led_green = gpio.get_pin(14).into_push_pull_output();
        let mut led_blue = gpio.get_pin(15).into_push_pull_output();
        let mut buzzer_1 = gpio.get_pin(16).into_push_pull_output();
        let mut buzzer_2 = gpio.get_pin(17).into_push_pull_output();

        // Inicializa LEDs e buzzer
        led_red.set_low().ok();
        led_green.set_low().ok();
        led_blue.set_low().ok();
        buzzer_1.set_low().ok();
        buzzer_2.set_low().ok();

        // Retorna o contexto com as variáveis locais
        (
            Shared {},
            Local {
                led_red,
                led_green,
                led_blue,
                buzzer_1,
                buzzer_2,
                delay,
            },
        )
    }

    #[task(local = [led_red, led_green, led_blue, buzzer_1, buzzer_2, delay])]
    fn blink(ctx: blink::Context) {
        // Exemplo de alternância simples entre LEDs
        let delay = &mut ctx.local.delay;
        
        ctx.local.led_red.set_high().ok();
        ctx.local.led_green.set_low().ok();
        ctx.local.led_blue.set_low().ok();
        delay.delay_ms(500_u16);

        ctx.local.led_red.set_low().ok();
        ctx.local.led_green.set_high().ok();
        ctx.local.led_blue.set_low().ok();
        delay.delay_ms(500_u16);

        ctx.local.led_red.set_low().ok();
        ctx.local.led_green.set_low().ok();
        ctx.local.led_blue.set_high().ok();
        delay.delay_ms(500_u16);
    }

    #[task(local = [buzzer_1, buzzer_2, delay])]
    fn buzz(ctx: buzz::Context) {
        // Exemplo de controle simples dos buzzers
        let delay = &mut ctx.local.delay;

        ctx.local.buzzer_1.set_high().ok();
        ctx.local.buzzer_2.set_low().ok();
        delay.delay_ms(1000_u16);

        ctx.local.buzzer_1.set_low().ok();
        ctx.local.buzzer_2.set_high().ok();
        delay.delay_ms(1000_u16);
    }
}
