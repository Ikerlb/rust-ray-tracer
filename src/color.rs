use crate::vec;
use crate::util::clamp;

pub type Color = vec::Vector<3>;

impl Color {
    pub fn new_color(r: f64, g: f64, b: f64) -> Color {
        vec::Vector {
            data: [r, g, b],
        }
    }
    
    pub fn r(&self) -> f64 {
        self.data[0]
    }

    pub fn g(&self) -> f64 {
        self.data[1]
    }

    pub fn b(&self) -> f64 {
        self.data[2]
    }

    pub fn ppm_color(&self, samples: f64) -> String {
        let r = (self.r() / samples).sqrt();
        let g = (self.g() / samples).sqrt();
        let b = (self.b() / samples).sqrt();

        let rd = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let gd = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let bd = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        format!("{} {} {}", rd, gd, bd)
    }
}
