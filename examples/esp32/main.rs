#![no_std]
#![no_main]
extern crate xgzp6897d;
extern crate esp_idf_svc;
extern crate alloc;

use xgzp6897d::XGZP6897D;
use embedded_hal::delay::DelayNs;
use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use esp_idf_svc::hal::task::block_on;

#[no_mangle]
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut device = XGZP6897D::new(i2c, xgzp6897d::DEVICE_ADDRESS, 4096f32);

    loop {
        match device.read_sensor() {
            Ok((pressure, temperature)) => {
                log::info!("Pressure: {:.3} Pa; Temperature: {:.2} °C", pressure, temperature);
            }
            Err(_) => log::error!("Failed to read XGZP6897D!"),
        }

        FreeRtos.delay_ms(1000);
    }
}
