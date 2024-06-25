//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio,
    i2c::{self, Config},
};
use embassy_time::Timer;
use gpio::{Level, Output};
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    info!("set up i2c ");
    let sda = p.PIN_0;
    let scl = p.PIN_1;

    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.init().unwrap();
    let _ = display.clear();

    loop {
        let _ = display.write_str("led on");
        info!("led on!");
        led.set_high();
        Timer::after_secs(1).await;

        let _ = display.write_str("led off");
        info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
