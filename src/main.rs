extern crate time;

extern crate wasp;
extern crate hardware;

extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

use wasp::motor::Motor;
use wasp::motor::StepperMotor;
use wasp::motor::StepperMotorConfig;

use hardware::pin::Pin;

mod simulator;
use simulator::SimulatedPins;
use simulator::SimulatedStepper;
use simulator::StepperDirection;
use simulator::SimulatedTime;

const MAX_X: f32 = 200.0;
const MIN_X: f32 = -200.0;
const MAX_Y: f32 = 200.0;
const MIN_Y: f32 = -200.0;
const MAX_Z: f32 = 200.0;
const MIN_Z: f32 = -200.0;

const GRID_STEP: f32 = 10.0;
const GRID_TICK: f32 = 1.0;

fn draw_axis(win: &mut Window) {

    // Draw Axis
    win.draw_line(
        &na::Point3::new(MIN_X, 0.0, 0.0),
        &na::Point3::new(MAX_X, 0.0, 0.0),
        &na::Point3::new(1.0, 0.0, 0.0),
    );

    let x_steps = ((MAX_X - MIN_X) / GRID_STEP) as i32;
    for x_step in 0..x_steps {
        let x = x_step as f32 * GRID_STEP + MIN_X;
        win.draw_line(
            &na::Point3::new(x, 0.0, -GRID_TICK),
            &na::Point3::new(x, 0.0, GRID_TICK),
            &na::Point3::new(1.0, 0.0, 0.0),
        )
    }


    // Y Axis
    win.draw_line(
        &na::Point3::new(0.0, 0.0, MIN_Y),
        &na::Point3::new(0.0, 0.0, MAX_Y),
        &na::Point3::new(0.0, 1.0, 0.0),
    );

    let y_steps = ((MAX_Y - MIN_Y) / GRID_STEP) as i32;
    for y_step in 0..y_steps {
        let y = y_step as f32 * GRID_STEP + MIN_X;
        win.draw_line(
            &na::Point3::new(-GRID_TICK, 0.0, y),
            &na::Point3::new(GRID_TICK, 0.0, y),
            &na::Point3::new(0.0, 1.0, 0.0),
        )
    }


    // Z Axis
    win.draw_line(
        &na::Point3::new(0.0, MIN_Z, 0.0),
        &na::Point3::new(0.0, MAX_Z, 0.0),
        &na::Point3::new(0.0, 0.0, 1.0),
    );

    let z_steps = ((MAX_Z - MIN_Z) / GRID_STEP) as i32;
    for z_step in 0..z_steps {
        let z = z_step as f32 * GRID_STEP + MIN_Z;
        win.draw_line(
            &na::Point3::new(-GRID_TICK, z, 0.0),
            &na::Point3::new(GRID_TICK, z, 0.0),
            &na::Point3::new(0.0, 0.0, 1.0),
        )
    }
}

fn main() {
    let mut window = Window::new("WASP Simulator");
    window.set_background_color(1.0, 1.0, 1.0);

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    //let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut pins = SimulatedPins {
        x_step: Pin(1),
        y_step: Pin(2),
        z_step: Pin(3),

        x_dir: Pin(4),
        y_dir: Pin(5),
        z_dir: Pin(6),
    };

    let time = SimulatedTime::new();

    let mut x_stepper_output = SimulatedStepper::new(&mut pins.x_step, &mut pins.x_dir);

    let x_stepper_config = StepperMotorConfig {
        min_travel: 0.0,
        max_travel: 200.0,

        steps_per_millimeter: 200,
        pulse_length: 100,
    };

    let x_stepper = StepperMotor::new(&mut x_stepper_output, &mut x_stepper_output.direction, &time, x_stepper_config);

    while window.render() {
        //c.prepend_to_local_rotation(&rot);
        draw_axis(&mut window);
        //c.set_local_transformation(na::convert(na::Translation3::from_vector(hardware.get_position())));
    }
}
