
pub mod stepper;
pub mod time;

use hardware::pin::Pin;

pub struct SimulatedPins {
    pub x_step: Pin<u8>,
    pub y_step: Pin<u8>,
    pub z_step: Pin<u8>,

    pub x_dir: Pin<u8>,
    pub y_dir: Pin<u8>,
    pub z_dir: Pin<u8>,
}
