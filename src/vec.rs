use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

// rand
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<const N: usize> {
    pub data: [f64; N],
}

impl<const N: usize> Vector<N> {


    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.data.iter().all(|&e| e.abs() < s)
    }

    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = rng.gen();
        }
        Vector { data }
    }

    pub fn reflect(&self, n: &Vector<N>) -> Vector<N> {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vector<N>, etai_over_etat: f64) -> Vector<N> {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn rand_range<R: Rng>(rng: &mut R, min: f64, max: f64) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = rng.gen_range(min..=max);
        }
        Vector { data }
    }

    pub fn rand_in_unit_sphere<R: Rng>(rng: &mut R) -> Self {
        loop {
            let p = Vector::rand_range(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_unit_vector<R: Rng>(rng: &mut R) -> Self {
        Self::rand_in_unit_sphere(rng).unit_vector()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    fn add_vec(&self, other: &Self) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }

    fn sub_vec(&self, other: &Self) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = self.data[i] - other.data[i];
        }
        Self { data }
    }

    fn scale(&self, scalar: f64) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = self.data[i] * scalar;
        }
        Self { data }
    }

    pub fn length_squared(&self) -> f64 {
        self.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.data.iter()
    }

    fn add_assign(&mut self, other: &Self) {
        for i in 0..N {
            self.data[i] += other.data[i];
        }
    }

    fn sub_assign(&mut self, other: &Self) {
        for i in 0..N {
            self.data[i] -= other.data[i];
        }
    }

    fn scale_mult_assign(&mut self, other: f64) {
        for i in 0..N {
            self.data[i] *= other;
        }
    }

    fn scale_div_assign(&mut self, other: f64) {
        for i in 0..N {
            self.data[i] /= other;
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn prod(&self, other: &Self) -> Self {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = self.data[i] * other.data[i];
        }
        Self { data }
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        self.scale(-1.0)
    }
}

impl<const N: usize> Neg for &Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        self.scale(-1.0)
    }
}

// &Vector + &Vector
impl<const N: usize> Add<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: &Vector<N>) -> Self::Output {
        self.add_vec(&other)
    }
}

// Vector + &Vector
impl<const N: usize> Add<&Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: &Vector<N>) -> Self::Output {
        self.add_vec(other)
    }
}

// &Vector + Vector
impl<const N: usize> Add<Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Self::Output {
        self.add_vec(&other)
    }
}

// Vector + Vector
impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Self::Output {
        self.add_vec(&other)
    }
}

// += &Vector
impl<const N: usize> AddAssign<&Vector<N>> for Vector<N> {
    fn add_assign(&mut self, other: &Vector<N>) {
        self.add_assign(&other)
    }
}

// += Vector
impl<const N: usize> AddAssign<Vector<N>> for Vector<N> {
    fn add_assign(&mut self, other: Vector<N>) {
        self.add_assign(&other)
    }
}

// &Vector - &Vector
impl<const N: usize> Sub<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: &Vector<N>) -> Self::Output {
        self.sub_vec(&other)
    }
}

// Vector - &Vector
impl<const N: usize> Sub<&Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: &Vector<N>) -> Self::Output {
        self.sub_vec(other)
    }
}

// &Vector - Vector
impl<const N: usize> Sub<Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: Vector<N>) -> Self::Output {
        self.sub_vec(&other)
    }
}

// Vector - Vector
impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: Vector<N>) -> Self::Output {
        self.sub_vec(&other)
    }
}

impl<const N: usize> SubAssign<&Vector<N>> for Vector<N> {
    fn sub_assign(&mut self, other: &Vector<N>) {
        self.sub_assign(&other)
    }
}

impl<const N: usize> SubAssign<Vector<N>> for Vector<N> {
    fn sub_assign(&mut self, other: Vector<N>) {
        self.sub_assign(&other)
    }
}

// &Vector * f64
impl<const N: usize> Mul<f64> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, scalar: f64) -> Self::Output {
        self.scale(scalar)
    }
}

// f64 * &Vector
impl<const N: usize> Mul<&Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Self::Output {
        other.scale(self)
    }
}

// Vector * f64
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, scalar: f64) -> Self::Output {
        self.scale(scalar)
    }
}

// f64 * Vector
impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Self::Output {
        other.scale(self)
    }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Self::Output {
        self.prod(&other)
    }
}

impl<const N: usize> Mul<Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Self::Output {
        self.prod(&other)
    }
}

impl<const N: usize> Mul<&Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Self::Output {
        self.prod(other)
    }
}

impl<const N: usize> Mul<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Self::Output {
        self.prod(other)
    }
}

impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, other: f64) {
        self.scale_mult_assign(other)
    }
}

// &Vector / f64
impl<const N: usize> Div<f64> for &Vector<N> {
    type Output = Vector<N>;

    fn div(self, scalar: f64) -> Self::Output {
        self.scale(1.0 / scalar)
    }
}

// Vector / f64
impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, scalar: f64) -> Self::Output {
        self.scale(1.0 / scalar)
    }
}

impl<const N: usize> DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, other: f64) {
        self.scale_div_assign(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }


    #[test]
    fn test_add() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let v1 = Vector::<3>::rand(&mut rng);
            let v2 = Vector::<3>::rand(&mut rng);
            let res = &v1 + &v2;

            for i in 0..3 {
                assert_eq!(res.data[i], v1.data[i] + v2.data[i]);
            }
        }
    }

    #[test]
    fn test_add_assign() {
        let mut rng = rand::thread_rng();

        for i in 0..100 {
            let mut v1 = Vector::<3>::rand(&mut rng);
            let v2 = Vector::<3>::rand(&mut rng);

            let res = &v1 + &v2;
            v1 += &v2;

            for i in 0..3 {
                assert_eq!(res.data[i], v1.data[i]);
            }
        }
    }


    #[test]
    fn test_sub() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let v1 = Vector::<3>::rand(&mut rng);
            let v2 = Vector::<3>::rand(&mut rng);
            let res = &v1 - &v2;

            for i in 0..3 {
                assert_eq!(res.data[i], v1.data[i] - v2.data[i]);
            }
        }
    }

    #[test]
    fn test_sub_assign() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let mut v1 = Vector::<3>::rand(&mut rng);
            let v2 = Vector::<3>::rand(&mut rng);
            let res = &v1 - &v2;

            v1 -= &v2;

            for i in 0..3 {
                assert_eq!(res.data[i], v1.data[i]);
            }
        }
    }


    #[test]
    fn test_scale_mult() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let v1 = Vector::<3>::rand(&mut rng);
            let scalar = rng.gen::<f64>();
            let res1 = &v1 * scalar;
            let res2 = scalar * &v1;

            for i in 0..3 {
                assert_eq!(res1.data[i], v1.data[i] * scalar);
                assert_eq!(res2.data[i], v1.data[i] * scalar);
            }
        }
    }

    #[test]
    fn test_scale_mult_assign() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let mut v1 = Vector::<3>::rand(&mut rng);
            let scalar = rng.gen::<f64>();
            let res = &v1 * scalar;

            v1 *= scalar;

            for i in 0..3 {
                assert_eq!(res.data[i], v1.data[i]);
            }
        }
    }

    #[test]
    fn test_scale_div_assign() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let mut v1 = Vector::<3>::rand(&mut rng);
            let scalar = rng.gen::<f64>();
            let res = &v1 / scalar;

            v1 /= scalar;

            for i in 0..3 {
                assert_delta!(res.data[i], v1.data[i], 0.0000001);
            }
        }
    }

    #[test]
    fn test_scale_div() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let v1 = Vector::<3>::rand(&mut rng);
            let scalar = rng.gen::<f64>();
            let res = &v1 / scalar;

            for i in 0..3 {
                assert_delta!(res.data[i], v1.data[i], 0.0000001);
            }
        }
    }
}
