use super::{OLED_PAGE, SSD1306, SSD1306_V2};
use embedded_hal::i2c::I2c as HalI2c;
impl<'a, I2C> SSD1306<'a, I2C>
where
    I2C: HalI2c,
{
    pub fn new_frame(&mut self) {
        if let Some(vram) = self.vram.as_mut() {
            for i in 0..OLED_PAGE as usize {
                let start = i * (self.col as usize + 1) + 1;
                let end = start + self.col as usize;
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
}

impl<'a, I2C: HalI2c> SSD1306_V2<'a, I2C> {
	pub fn new_frame(&mut self){
		for i in 0..(self.row/OLED_PAGE as u16) as usize {
			let start = i* (self.col as usize +1) +1;
			let end = start+ self.col as usize;
			self.vram.ram[start..end].fill(0);
		}
	}
	pub fn show_frame(&mut self){
		let ram = self.vram.ram;
		for i in 0..(self.row / OLED_PAGE as u16) as usize {
			self.send_cmd(0xB0 + i as u8);
            self.send_cmd(0x02);
            self.send_cmd(0x10);
			let start = i * (self.col as usize + 1);
            let end = start + self.col as usize;
			self.send(&ram[start..=end]);
		}
	}
}