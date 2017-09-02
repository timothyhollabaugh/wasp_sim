

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
