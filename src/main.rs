use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use smart_leds::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrbw32, LedPixelEsp32Rmt, RGBW8};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio7).unwrap();
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, led_pin).unwrap();
    // let mut led = WS2812rmt::new(peripherals.pins.gpio2, peripherals.rmt.channel0);

    loop {
        // led_pin.set_low().unwrap();
        // log::info!("LED on");
        // FreeRtos::delay_ms(1000);
        // led_pin.set_high().unwrap();
        // log::info!("LED off");
        // FreeRtos::delay_ms(1000);
        let pixels = std::iter::repeat(RGBW8::new_alpha(255, 0, 0, White(0))).take(25);
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(1000);
        let pixels = std::iter::repeat(RGBW8::new_alpha(0, 255, 0, White(0))).take(25);
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(1000);
        let pixels = std::iter::repeat(RGBW8::new_alpha(0, 0, 255, White(0))).take(25);
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(1000);
        let pixels = std::iter::repeat(RGBW8::new_alpha(128, 128, 128, White(0))).take(25);
        ws2812.write(pixels).unwrap();
        FreeRtos::delay_ms(1000);
    }
}
