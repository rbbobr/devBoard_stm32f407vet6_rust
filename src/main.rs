// #![deny(unsafe_code)]
#![no_main]
#![no_std]


//
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32;
use embassy_stm32::Config;
use embassy_stm32::Peripherals;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
//


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config: Config = Config::default();
    let _p:Peripherals = embassy_stm32::init(config);

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
