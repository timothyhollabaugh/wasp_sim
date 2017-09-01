
use wasp::motor::Direction;
use hardware::peripherals::digital_io::DigitalOutput;
use hardware::peripherals::digital_io::DigitalValue;

struct SimulatedStepper {
    position: f32,
    direction: Direction,
}
