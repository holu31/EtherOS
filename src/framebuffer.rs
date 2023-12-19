use crate::multiboot::MultibootHeader;
use crate::{log, serial_print};
use font8x8::{UnicodeFonts, BASIC_UNICODE};

#[derive(Debug)]
pub struct Framebuffer {
    pub buffer: *mut u8,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,
    pub x: usize,
    pub y: usize
}

impl Framebuffer {
    
    pub unsafe fn new(mboot: *const MultibootHeader) -> Self {
        let fb = Framebuffer {
            buffer: (*mboot).framebuffer_addr as *mut u8,
            width: (*mboot).framebuffer_width as usize,
            height: (*mboot).framebuffer_height as usize,
            pitch: (*mboot).framebuffer_pitch as usize,
            bpp: (*mboot).framebuffer_bpp as usize,
            x: 0, y: 0
        };
        log!("{:#?}", fb);

        fb
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.x = 0;
            self.y += 8
        }
        else if let Some(glyph) = BASIC_UNICODE.get(c as usize) {
            let line = glyph.byte_array();
            for y in 0..8 {
                for x in 0..8 {
                    if (line[y] >> x) & 1 == 1 {
                        self.pixel(self.x + x, self.y + y, 0x000000);
                    }
                }
            }
            self.x += 8;
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    fn pixel_pos(&mut self, x: usize, y: usize) -> usize {
        x * (self.bpp / 8) + y * self.pitch
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width && y >= self.height {
            return;
        }

        let pos = self.pixel_pos(x, y) as isize;
        
        unsafe {
            *self.buffer.offset(pos).as_mut().unwrap() = color as u8;
            *self.buffer.offset(pos + 1).as_mut().unwrap() = (color >> 8) as u8;
            *self.buffer.offset(pos + 2).as_mut().unwrap() = (color >> 16) as u8;
        }
    }


}