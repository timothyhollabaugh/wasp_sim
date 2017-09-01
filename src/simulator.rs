
use std::time::Instant;

use wasp::motor::Direction;
use hardware::peripherals::digital_io::DigitalOutput;
use hardware::peripherals::digital_io::DigitalValue;
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

pub struct SimulatedStepper<'a> {
    step: i32,
    pin: &'a mut Pin<u8>,
    val: DigitalValue,
    pub direction: StepperDirection<'a>,
}

impl<'a> SimulatedStepper<'a> {
    pub fn new(step_pin: &'a mut Pin<u8>, dir_pin: &'a mut Pin<u8>) -> SimulatedStepper<'a> {
        SimulatedStepper {
            step: 0,
            pin: step_pin,
            val: DigitalValue::Low,
            direction: StepperDirection::new(dir_pin),
        }
    }

    pub fn step(&self) -> i32 {
        self.step
    }
}

impl<'a> DigitalOutput for SimulatedStepper<'a> {
    fn write(&mut self, val: DigitalValue) {
        if self.val == DigitalValue::Low && val == DigitalValue::High {
            match self.direction.direction() {
                Direction::Forward => self.step += 1,
                Direction::Backward => self.step -= 1,
            }
        }
        self.val = val;
    }

    fn read(&self) -> DigitalValue {
        self.val
    }
}

pub struct StepperDirection<'a> {
    pin: &'a mut Pin<u8>,
    direction: Direction,
}

impl<'a> StepperDirection<'a> {
    pub fn new(pin: &'a mut Pin<u8>) -> StepperDirection<'a> {
        StepperDirection {
            pin: pin,
            direction: Direction::Forward,
        }
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

impl<'a> DigitalOutput for StepperDirection<'a> {
    fn write(&mut self, val: DigitalValue) {
        self.direction = match val {
            DigitalValue::High => Direction::Forward,
            DigitalValue::Low => Direction::Backward,
        }
    }
    fn read(&self) -> DigitalValue {
        match self.direction {
            Direction::Forward => DigitalValue::High,
            Direction::Backward => DigitalValue::Low,
        }
    }
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
        now.as_secs() as u32 * 1000_u32 * 1000_u32 + now.subsec_nanos() / 1000_u32
    }

    fn delay(&self, delay: u32){
        unimplemented!();
    }
}
