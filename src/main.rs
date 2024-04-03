use nannou::prelude::*;
use physics::calculate_gravitational_force;
use rand::Rng;

mod celestial_body;
mod physics;
mod vec3;

use celestial_body::CelestialBody;
use vec3::Vec3;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    bodies: Vec<CelestialBody>,
    zoom: f32,       // Zoom level
    time: f64,       // Current simulation time
    forward: bool,   // True if time is moving forward, false if backward
    paused: bool,    // True if the simulation is paused
    delta_time: f64, // Speed of simulation
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 768)
        .view(view)
        .key_pressed(key_pressed) // Handle key presses
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let mut bodies = Vec::new();

    // Create three suns with random positions and masses
    for _ in 0..3 {
        bodies.push(CelestialBody::new(
            Vec3::new(
                rng.gen_range(-1.0e11..1.0e11),
                rng.gen_range(-1.0e11..1.0e11),
                rng.gen_range(-1.0e11..1.0e11),
            ),
            Vec3::new(0.0, 0.0, 0.0),
            rng.gen_range(1.0e30..2.0e30),
        ));
    }

    // Add a planet
    bodies.push(CelestialBody::new(
        Vec3::new(
            rng.gen_range(-1.0e11..1.0e11),
            rng.gen_range(-1.0e11..1.0e11),
            rng.gen_range(-1.0e11..1.0e11),
        ),
        Vec3::new(0.0, 0.0, 0.0),
        rng.gen_range(1.0e24..1.0e25),
    ));
    Model {
        bodies,
        zoom: 1.0,          // Starting zoom level, adjust as necessary
        time: 0.0,          // Starting simulation time
        forward: true,      // Initially moving forward in time
        paused: false,      // Initially not paused
        delta_time: 1000.0, // Initial speed of simulation
    }
}

fn update_positions_and_velocities(bodies: &mut [CelestialBody], delta_time: f64) {
    let forces = bodies
        .iter()
        .enumerate()
        .map(|(i, body)| {
            bodies
                .iter()
                .enumerate()
                .fold(Vec3::new(0.0, 0.0, 0.0), |acc, (j, other)| {
                    if i != j {
                        acc + calculate_gravitational_force(body, other)
                    } else {
                        acc
                    }
                })
        })
        .collect::<Vec<Vec3>>();

    // Update velocities and positions based on the accumulated forces
    for (i, body) in bodies.iter_mut().enumerate() {
        let acceleration = forces[i].multiply_scalar(1.0 / body.mass);
        // log to cli
        println!("Velocity before: {:?} {:?}", body.mass, body.velocity);
        println!("Position before: {:?} {:?}", body.mass, body.position);
        body.velocity = body.velocity + acceleration.multiply_scalar(delta_time);
        body.position = body.position + body.velocity.multiply_scalar(delta_time);
        println!("Velocity after: {:?} {:?}", body.mass, body.velocity);
        println!("Position after: {:?} {:?}", body.mass, body.position);
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.paused {
        return; // Do not update the simulation if paused
    }

    let delta_time = if model.forward {
        1000.0 * model.delta_time
    } else {
        -1000.0 * model.delta_time
    };
    update_positions_and_velocities(&mut model.bodies, delta_time);

    // Any additional update logic would go here. For example, you could:
    // - Check for mouse input to add/move bodies
    // - Adjust simulation parameters based on other user inputs
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => {
            model.zoom *= 1.1; // Zoom in
            println!("Zooming in. Current zoom: {}", model.zoom);
        }
        Key::Down => {
            model.zoom /= 1.1; // Zoom out
            println!("Zooming out. Current zoom: {}", model.zoom);
        }
        Key::Right => {
            model.forward = true; // Move time forward
            println!("Moving time forward");
        }
        Key::Left => {
            model.forward = false; // Move time backward
            println!("Moving time backward");
        }
        Key::Key1 => {
            model.delta_time = 1.0; // Set speed to 1
            println!("Speed set to 1. Delta time: {}", model.delta_time);
        }
        Key::Key2 => {
            model.delta_time = 5.0; // Set speed to 2
            println!("Speed set to 2. Delta time: {}", model.delta_time);
        }
        Key::Key3 => {
            model.delta_time = 10.0; // Set speed to 3
            println!("Speed set to 3. Delta time: {}", model.delta_time);
        }
        Key::Space => {
            // Assuming you have a `paused` field in your Model to toggle
            model.paused = !model.paused;
            println!("Simulation paused: {}", model.paused);
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for body in &model.bodies {
        let is_planet = body.mass < 1.0e28; // Assuming bodies with mass less than this threshold are planets
        let size = if is_planet {
            3.0 * model.zoom
        } else {
            10.0 * model.zoom
        }; // Adjust these sizes as needed

        draw.ellipse()
            .x_y(
                body.position.x as f32 / 1e10 * model.zoom,
                body.position.y as f32 / 1e10 * model.zoom,
            )
            .w_h(size, size)
            .rgba(1.0, 1.0, 1.0, 1.0);
    }

    draw.to_frame(app, &frame).unwrap();
}
