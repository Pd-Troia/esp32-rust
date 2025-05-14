


use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use std::{thread, time::Duration};
pub fn blink_led() {   
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();   
    let peripherals = Peripherals::take().unwrap();
    let mut led1 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    let mut led2 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    loop {
        led1.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        led1.set_low().unwrap();
        led2.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        led2.set_low().unwrap();
    }     
}
 