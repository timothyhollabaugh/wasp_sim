use std::thread::sleep;
use std::time::Duration;
use time;

use hardware::peripherals::digital_io::DigitalOutput;
use hardware::peripherals::digital_io::DigitalValue;
use hardware::peripherals::time::Time;
use hardware::pin::Pin;

use na;

#[derive(Debug)]
pub struct Simulator {
    pub x_pos: Pin<u8>,
    pub y_pos: Pin<u8>,
    pub z_pos: Pin<u8>,

    pub x_steps_per_mm: Pin<u8>,
    pub y_steps_per_mm: Pin<u8>,
    pub z_steps_per_mm: Pin<u8>,

    pub x_step_pin: Pin<u8>,
    pub y_step_pin: Pin<u8>,
    pub z_step_pin: Pin<u8>,

    pub x_dir_pin: Pin<u8>,
    pub y_dir_pin: Pin<u8>,
    pub z_dir_pin: Pin<u8>,

    pub pins: [DigitalValue; 32],
}

impl Simulator {
    pub fn get_position(&self) -> na::Vector3<f32> {
        na::Vector3::new(self.x_pos, self.y_pos, self.z_pos)
    }
}

impl DigitalOutput for Simulator {
    fn write(&mut self, val: PinState) {
        if self.pins[pin as usize] == PinState::Low && val == PinState::High {
            match pin {
                p if (p == self.x_step_pin) => self.x_pos += 1.0 / self.x_steps_per_mm,
                p if (p == self.y_step_pin) => self.y_pos += 1.0 / self.y_steps_per_mm,
                p if (p == self.z_step_pin) => self.z_pos += 1.0 / self.z_steps_per_mm,
                _ => {}
            }
        }

        self.pins[pin as usize] = val;
    }

    fn read(&self, pin: u8) -> PinState {
        self.pins[pin as usize]
    }
}

impl HardwareTime for Simulator {
    fn delay(&self, micros: u32) {
        sleep(Duration::new(0, micros*1000));
    }

    fn now(&self) -> u32 {
        let nowtime = time::now().to_timespec();

        (nowtime.sec * 1000000) as u32 + (nowtime.nsec / 1000) as u32
    }
}
