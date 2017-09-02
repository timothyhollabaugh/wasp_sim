
pub mod stepper;

use std::time::Instant;

use hardware::peripherals::time::Time;
use hardware::pin::Pin;

pub struct SimulatedPins {
    pub x_step: Pin<u8>,
    pub y_step: Pin<u8>,
    pub z_step: Pin<u8>,

    pub x_dir: Pin<u8>,
    pub y_dir: Pin<u8>,
    pub z_dir: Pin<u8>,
}

pub struct SimulatedTime {
    start: Instant,
}

impl SimulatedTime {
    pub fn new() -> SimulatedTime {
        SimulatedTime {
            start: Instant::now(),
        }
    }
}

impl Time for SimulatedTime {
    fn now(&self) -> u32{
        let now = self.start.elapsed();
        let now_micros = now.as_secs() as u32 * 1000_u32 * 1000_u32 + now.subsec_nanos() / 1000_u32;
        //println!("Now: {}", now_micros);
        now_micros
    }

    fn delay(&self, delay: u32){
        unimplemented!();
    }
}
