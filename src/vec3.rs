use crate::vec;

pub type Vec3 = vec::Vector<3>;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        vec::Vector {
            data: [x, y, z],
        }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}
