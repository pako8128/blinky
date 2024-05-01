#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::Timer;

#[embassy_executor::task]
async fn blink(mut led: Output<'static, AnyPin>) {
    loop {
        defmt::info!("blinking!");
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let plat = embassy_stm32::init(Default::default());
    let led = Output::new(AnyPin::from(plat.PA5), Level::Low, Speed::Low);

    spawner.spawn(blink(led)).unwrap();
}
