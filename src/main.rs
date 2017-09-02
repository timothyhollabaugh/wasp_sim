extern crate time;

extern crate wasp;
extern crate hardware;

extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, Point3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;

use wasp::motor::Motor;
use wasp::motor::Direction;
use wasp::motor::StepperDriver;
use wasp::motor::StepperDriverConfig;

use hardware::pin::Pin;

mod simulator;
use simulator::SimulatedPins;
use simulator::SimulatedStepper;
use simulator::StepOutput;
use simulator::DirectionOutput;
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

    let mut camera = ArcBall::new(Point3::from_coordinates(Vector3::new(-10.0, 10.0, 10.0)), Point3::origin());

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

    let x_simulated_stepper = SimulatedStepper::new();

    let mut x_step_output = StepOutput::new(&mut pins.x_step, &x_simulated_stepper);
    let mut x_direction_output = DirectionOutput::new(&mut pins.x_dir, &x_simulated_stepper);

    let x_stepper_config = StepperDriverConfig {
        min_travel: 0.0,
        max_travel: 200.0,

        steps_per_millimeter: 200,
        pulse_length: 1,
    };

    let mut x_stepper = StepperDriver::new(&mut x_step_output, &mut x_direction_output, &time, x_stepper_config);

    x_stepper.set_direction(Direction::Forward);
    x_stepper.set_velocity(60.0);
    println!("Set velocity to: {}", x_stepper.get_velocity());
    println!("Set microseconds per step to: {}", x_stepper.get_microseconds_per_step());

    while window.render_with_camera(&mut camera) {
        //c.prepend_to_local_rotation(&rot);
        draw_axis(&mut window);
        x_stepper.update();
        //println!("Stepper pos: {}", x_stepper.get_position());
        //println!("Simulated Stepper step: {}", x_simulated_stepper.get_step());
        c.set_local_transformation(na::convert(na::Translation3::from_vector(Vector3::new((x_simulated_stepper.get_step() as f64) / (x_stepper_config.steps_per_millimeter as f64), 0.0, 0.0))));
    }
}
