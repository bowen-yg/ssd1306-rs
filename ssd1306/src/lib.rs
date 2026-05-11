#![allow(warnings)]
#![no_std]
#![no_main]
use core::ops::{Index, IndexMut};
use embedded_hal::i2c::I2c as HalI2c;
const OLED_PAGE: u8 = 8;
pub mod full;
pub mod part;

struct Vram<'a> {
    ram: &'a mut [u8],
    col: u16,
}
pub struct SSD1306<'a, I2C> {
    i2c: I2C,
    addr: u8,
    pub row: u8,
    pub col: u8,
    vram: Option<&'a mut [u8]>,
}

pub struct SSD1306_V2<'a, I2C> {
    i2c: I2C,
    addr: u8,
    row: u16,
    pub col: u16,
    vram: Vram<'a>,
}

impl<'a, I2C> SSD1306<'a, I2C>
where
    I2C: HalI2c,
{
    #[inline]
    fn send(&mut self, data: &[u8]) -> Result<(), I2C::Error> {
        self.i2c.write(self.addr, data)
    }
    #[inline]
    fn send_cmd(&mut self, cmd: u8) {
        let buf: [u8; 2] = [0, cmd];
        self.send(&buf);
    }

    pub fn new(i2c: I2C, addr: u8, row: u8, col: u8, vram: Option<&'a mut [u8]>) -> Self {
        Self {
            i2c,
            addr,
            row,
            col,
            vram,
        }
    }
    pub fn init(&mut self) {
        if let Some(vram) = self.vram.as_mut() {
            for i in 0..OLED_PAGE as usize {
                vram[i * (self.col + 1) as usize] = 0x40;
            }
        }
        let buf: [u8; _] = [
            0xAE, 0x20, 0x02, 0xB0, 0xC8, 0x00, 0x10, 0x40, 0x81, 0xDF, 0xA1, 0xA6, 0xA8, 0x3F,
            0xA4, 0xD3, 0x00, 0xD5, 0xF0, 0xD9, 0x22, 0xDA, 0x12, 0xDB, 0x20, 0x8D, 0x14,
        ];
        for cmd in buf {
            self.send_cmd(cmd);
        }
        self.new_frame();
        self.show_frame();
        self.send_cmd(0xAF);
    }
    pub fn draw_point(&mut self, x: i16, y: i16) {
        if x < 0 || x > self.col as i16 - 1 {
            return;
        };
        if y < 0 || y > self.row as i16 - 1 {
            return;
        }
        if let Some(vram) = self.vram.as_mut() {
            let page = (y / 8) as usize;
            let bit = (y % 8) as u8;
            let addr = page * (self.col as usize + 1) + x as usize + 1;
            if addr < vram.len() {
                vram[addr] |= 1u8 << bit;
            }
        }
    }
}

impl<'a> Index<(usize, usize)> for Vram<'a> {
    type Output = u8;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.ram[row * self.col as usize + col]
    }
}

impl<'a> IndexMut<(usize, usize)> for Vram<'a> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.ram[row * self.col as usize + col]
    }
}

impl<'a, I2C: HalI2c> SSD1306_V2<'a, I2C> {
    #[inline(always)]
    fn send(&mut self, data: &[u8]) -> Result<(), I2C::Error> {
        self.i2c.write(self.addr, data)
    }
    #[inline]
    fn send_cmd(&mut self, cmd: u8) {
        let buf: [u8; 2] = [0, cmd];
        self.send(&buf);
    }
    pub fn new(i2c: I2C, addr: u8, row: u16, col: u16, vram: &'a mut [u8]) -> Self {
        Self {
            i2c,
            addr,
            row,
            col,
            vram: Vram {
                ram: vram,
                col: col + 1,
            },
        }
    }
    pub fn init(&mut self) {
        for i in 0..(self.row / OLED_PAGE as u16) as usize {
            self.vram[(i, 0)] = 0x40;
        }
        let buf: [u8; _] = [
            0xAE, 0x20, 0x02, 0xB0, 0xC8, 0x00, 0x10, 0x40, 0x81, 0xDF, 0xA1, 0xA6, 0xA8, 0x3F,
            0xA4, 0xD3, 0x00, 0xD5, 0xF0, 0xD9, 0x22, 0xDA, 0x12, 0xDB, 0x20, 0x8D, 0x14, 0xAF,
        ];
        for cmd in buf {
            self.send_cmd(cmd);
        }
    }
    pub fn draw_pixel(&mut self, x: i16, y: i16) {
        if x < 0 || x > self.col as i16 - 1 {
            return;
        }
        if y < 0 || y > self.row as i16 - 1 {
            return;
        }
        let page = (y / OLED_PAGE as i16) as usize;
        let bit = (y % 8) as u8;
        self.vram[(page, x as usize + 1)] |= 1u8 << bit;
    }
}
