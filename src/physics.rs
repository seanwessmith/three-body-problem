use crate::celestial_body::CelestialBody;
use crate::vec3::Vec3;

const G: f64 = 6.67430e-11; // Gravitational constant

pub fn calculate_gravitational_force(body1: &CelestialBody, body2: &CelestialBody) -> Vec3 {
    let distance_vector = body1.position.subtract(&body2.position);
    let distance = distance_vector.magnitude();
    if distance > 0.0 {
        let force_magnitude = G * body1.mass * body2.mass / distance.powi(2);
        distance_vector
            .normalize()
            .multiply_scalar(-force_magnitude)
    } else {
        Vec3::new(0.0, 0.0, 0.0) // Avoid division by zero
    }
}
