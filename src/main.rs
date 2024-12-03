// #![deny(unsafe_code)]
#![no_main]
#![no_std]


//
use defmt::info;
use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32;
use embassy_stm32::Config;
use embassy_stm32::Peripherals;

use embassy_time::Delay;
use embassy_time::Timer;
use embassy_time::Duration;

use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::PinState;
use {defmt_rtt as _, panic_probe as _};
//
//--------------------------------inludes
include!("rcc.rs");
//--------------------------------inludes


// #[cortex_m_rt::entry]
// fn main() -> ! {    
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
//rcc n system config
    let config: Config =  use_rcc::rcc_init();
    let p:Peripherals = embassy_stm32::init(config);
// gpio config 
    //Leds
    let mut led1: Output<'_, _> = Output::new(p.PE13, Level::High, Speed::High);
    let mut led2: Output<'_, _> = Output::new(p.PE14, Level::High, Speed::High);
    let mut led3: Output<'_, _> = Output::new(p.PE15, Level::High, Speed::High);
    //Buttons
    let btn1: Input<'_, _> = Input::new(p.PE10, Pull::None);
    let btn2: Input<'_, _> = Input::new(p.PE11, Pull::None);
    let btn3: Input<'_, _> = Input::new(p.PE12, Pull::None);

// tasks start
    (_spawner.spawn( demo_sleep_seconds() )).unwrap();  //task for delay test
    
// gpio config end
    loop {
        println!("Hello_World!");

        led1.set_state( PinState::from(btn1.is_high()) ).unwrap();
        led2.set_state( PinState::from(btn2.is_high()) ).unwrap();
        led3.set_state( PinState::from(btn3.is_high()) ).unwrap();
        
    //работа с задержкой 
        Timer::after(Duration::from_secs(1)).await;
    //вывод
        info!("task: main task -> sec = {}", embassy_time::Instant::now().as_secs());
        info!("btns state : btn1 = {}, btn2 = {}, btn3 = {}", btn1.is_high(), btn2.is_high(), btn3.is_high());
    }
}



#[embassy_executor::task]
async fn demo_sleep_seconds() {

    let mut delay: Delay = Delay;
    // suspend this task for one second.
    loop {
    //-----------------------async delay-----------------------
        Timer::after(Duration::from_secs(1)).await;
        // Timer::after_secs(1).await;
    //-----------------------blocked delay-----------------------    
        delay.delay_ms(500);
        // embassy_time::block_for(Duration::from_millis(500));

        info!("task: Demo sleep ->  msec = {}", embassy_time::Instant::now().as_millis());
    }
}
