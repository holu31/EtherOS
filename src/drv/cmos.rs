use crate::io::ports::*;

const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

pub struct RealTimeClock {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: usize
}

impl RealTimeClock {
    pub fn new() -> Self {
        Self {
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0
        }
    }

    unsafe fn read(&mut self, reg: u8) -> u8 {
        outb(CMOS_ADDRESS, reg);
        inb(CMOS_DATA)
    }

    unsafe fn get_update_in_progress_flag(&mut self) -> u8 {
        outb(CMOS_ADDRESS, 0x0A);
        inb(CMOS_DATA) & 0x80
    }

    unsafe fn read_into_rtc(&mut self) {
        while self.get_update_in_progress_flag() != 0 {
            self.second = self.read(0x00);
            self.minute = self.read(0x02);
            self.hour = self.read(0x04);
            self.day = self.read(0x07);
            self.month = self.read(0x08);
            self.year = self.read(0x09) as usize;
        }
    }

    pub unsafe fn read_rtc(&mut self) {
        self.read_into_rtc();

		let mut last_second;
		let mut last_minute;
		let mut last_hour;
		let mut last_day;
		let mut last_month;
		let mut last_year;

		loop {
			last_second = self.second;
			last_minute = self.minute;
			last_hour = self.hour;
			last_day = self.day;
			last_month = self.month;
			last_year = self.year;

			self.read_into_rtc();

			if last_second != self.second
				|| last_minute != self.minute
				|| last_hour != self.hour
				|| last_day != self.day
				|| last_month != self.month
				|| last_year != self.year
			{
				break;
			}
		}

		let register_b = self.read(0x0B);

		if (register_b & 0x04) == 0 {
			self.second = (self.second & 0x0F) + ((self.second / 16) * 10);
			self.minute = (self.minute & 0x0F) + ((self.minute / 16) * 10);
			self.hour = ((self.hour & 0x0F) + (((self.hour & 0x70) / 16) * 10)) | (self.hour & 0x80);
			self.day = (self.day & 0x0F) + ((self.day / 16) * 10);
			self.month = (self.month & 0x0F) + ((self.month / 16) * 10);
			self.year = (self.year & 0x0F) + ((self.year / 16) * 10);
		}

		if ((register_b & 0x02) == 0) && ((self.hour & 0x80) != 0) {
			self.hour = ((self.hour & 0x7F) + 12) % 24;
		}
    }
}