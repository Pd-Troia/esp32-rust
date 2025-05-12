use esp_idf_hal::prelude::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use std::{thread, time::Duration};
fn main() {   
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();   
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    led.set_high().unwrap(); 
    thread::sleep(Duration::from_millis(3000));
}
