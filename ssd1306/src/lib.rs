#![allow(warnings)]
#![no_std]
#![no_main]
use embedded_hal::i2c::I2c as HalI2c;

const OLED_PAGE: u8 = 8;
const OLED_ROW: u8 = 8 * OLED_PAGE;
const OLED_COL: u8 = 128;

pub struct SSD1306<'a, I2C> {
    i2c: I2C,
    addr: u8,
    pub row: u8,
    pub col: u8,
    vram: Option<&'a mut [u8]>,
}

impl<'a, I2C> SSD1306<'a, I2C>
where
    I2C: HalI2c,
{
    #[inline]
    fn send(&mut self, data: &[u8]) -> Result<(), I2C::Error> {
        self.i2c.write(self.addr, data)
    }
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

    pub fn new_frame(&mut self) {
        if let Some(vram) = self.vram.as_mut() {
            for i in 0..OLED_PAGE as usize {
                let start = i*(self.col as usize +1)+1;
                let end = start+self.col as usize;
                if end <= vram.len() {
                    vram[start..end].fill(0);
                }
            }
        }
    }
    pub fn show_frame(&mut self) {
        if let Some(mut vram) = self.vram.take() {
            for i in 0..OLED_PAGE as usize {
                self.send_cmd(0xB0 + i as u8);
                self.send_cmd(0x02);
                self.send_cmd(0x10);
                let start = i * (self.col as usize + 1);
                let end = start + self.col as usize;
                if end < vram.len() {
                    self.send(&vram[start..=end]);
                }
            }
            self.vram = Some(vram);
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
