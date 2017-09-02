
use std::cell::Cell;
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

pub struct SimulatedStepper {
    step: Cell<i32>,
    direction: Cell<Direction>,
} 

impl SimulatedStepper {
    pub fn new() -> SimulatedStepper {
        SimulatedStepper {
            step: Cell::default(),
            direction: Cell::default(),
        }
    }

    pub fn step(&self) {
        //println!("Simulated Stepper Stepping!");
        let step = self.step.get();
        let direction = self.direction.get();
        self.step.set(step + direction as i32);
    }

    pub fn get_step(&self) -> i32 {
        self.step.get()
    }

    pub fn set_direction(&self, direction: Direction) {
        self.direction.set(direction);
    }
}

pub struct StepOutput<'a>  {
    stepper: &'a SimulatedStepper,
    val: DigitalValue,
    pin: &'a mut Pin<u8>,
}

impl<'a> StepOutput<'a> {
    pub fn new(pin: &'a mut Pin<u8>, stepper: &'a SimulatedStepper) -> StepOutput<'a> {
        StepOutput {
            stepper: stepper,
            val: DigitalValue::Low,
            pin: pin,
        }
    }
}

impl<'a> DigitalOutput for StepOutput<'a> {
    fn write(&mut self, val: DigitalValue) {
        //println!("StepOutput Write: {:?}", val);
        if self.val == DigitalValue::Low && val == DigitalValue::High {
            self.stepper.step();
        }
        self.val = val;
    }

    fn read(&self) -> DigitalValue {
        self.val
    }
}

pub struct DirectionOutput<'a> {
    stepper: &'a SimulatedStepper,
    val: DigitalValue,
    pin: &'a mut Pin<u8>,
}

impl<'a> DirectionOutput<'a> {
    pub fn new(pin: &'a mut Pin<u8>, stepper: &'a SimulatedStepper) -> StepOutput<'a> {
        StepOutput {
            stepper: stepper,
            val: DigitalValue::Low,
            pin: pin,
        }
    }
}

impl<'a> DigitalOutput for DirectionOutput<'a> {
    fn write(&mut self, val: DigitalValue) {
        match val {
            DigitalValue::High => self.stepper.set_direction(Direction::Forward),
            DigitalValue::Low => self.stepper.set_direction(Direction::Backward),
        }
        self.val = val;
    }

    fn read(&self) -> DigitalValue {
        self.val
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
        let now_micros = now.as_secs() as u32 * 1000_u32 * 1000_u32 + now.subsec_nanos() / 1000_u32;
        //println!("Now: {}", now_micros);
        now_micros
    }

    fn delay(&self, delay: u32){
        unimplemented!();
    }
}
