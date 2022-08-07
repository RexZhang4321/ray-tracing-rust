use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(e1: f32, e2: f32, e3: f32) -> Vec3 {
        Vec3 {x: e1, y: e2, z: e3}
    }

    pub fn length_squared(self: Vec3) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self: Vec3) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self: Vec3, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // The cross product of two vectors is the third vector that is perpendicular to the two original vectors
    pub fn cross(self: Vec3, v: Vec3) -> Vec3 {
        Vec3 { x: self.y * v.z - self.z * v.y, y: self.z * v.x - self.x * v.z, z: self.x * v.y - self.y * v.x }
    }

    pub fn unit_vector(self: Vec3) -> Vec3 {
        self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }
}

// this is for &v1 + &v2
impl<'a, 'b> ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

// this is for v1 + v2
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {x: 1.0 / rhs * self.x, y: 1.0 / rhs * self.y, z: 1.0 / rhs * self.z}
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {

    use crate::Vec3;

    #[test]
    fn test_vec3_add() {
        let v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        let v2: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(&v1 + &v2, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(v1 + v2, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(v1, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        let v2: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v1 += v2;
        assert_eq!(v1, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_sub() {
        let v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        let v2: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 - v2, Vec3::new(1.0, 0.0, -1.0));
        assert_eq!(v1, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        let v2: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(1.0, 0.0, -1.0));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_mul() {
        let v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        let v2: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 * 2.0, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(2.0 * v1, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(v1.dot(v2), 12.0);
        assert_eq!(v1.cross(v2), Vec3::new(2.0, -4.0, 2.0));

        assert_eq!(v1, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn test_vec3_div() {
        let v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v1 / 2.0, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(v1, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut v1: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(1.0, 1.0, 1.0));
    }
}