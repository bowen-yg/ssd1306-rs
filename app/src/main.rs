#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{self as hal, i2c::I2c, prelude::*, rcc};
const OLED_ADDR: u8 = 0x3C;
use ssd1306::{ SSD1306_V2};
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .hclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(12.MHz()),
        &mut flash.acr,
    );
    let gpiob = dp.GPIOB.split(&mut rcc);
    let scl = gpiob.pb6;
    let sda = gpiob.pb7;
    let iic = I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
    );
    let mut vram = [0u8; 8 * 129];
    // let mut lcd = SSD1306::new(iic, OLED_ADDR, 64, 128, Some(&mut vram[..]));
    let mut lcd=SSD1306_V2::new(iic, OLED_ADDR, 64, 128, &mut vram);
    let mut x: i16 = 5;
    let mut step: i16 = 2;
    lcd.init();
    loop {
        lcd.new_frame();
        for y in 10..=60 {
            lcd.draw_pixel(x, y);
        }
        if x < 0 {
            step = 2;
        } else if x> lcd.col as i16 {
            step = -2;
        }
        x += step;
        lcd.show_frame();
    }
}
