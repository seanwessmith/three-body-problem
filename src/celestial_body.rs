use crate::vec3::Vec3;

pub struct CelestialBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
}

impl PartialEq for CelestialBody {
    fn eq(&self, other: &Self) -> bool {
        // Assuming the uniqueness of a celestial body can be determined by its position and mass
        // Adjust the criteria based on your application's needs
        self.position == other.position && self.mass == other.mass
    }
}

impl CelestialBody {
    pub fn new(position: Vec3, velocity: Vec3, mass: f64) -> Self {
        CelestialBody {
            position,
            velocity,
            mass,
        }
    }
}
