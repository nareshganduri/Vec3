use std::fmt;

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// create a new Vec3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    /// adds the vectors together
    pub fn add(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// computes the dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// computes the cross product
    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    /// computes the magnitude
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    /// computes a normalized version of the vector. The result is a unit
    /// vector
    pub fn unit(&self) -> Self {
        let mag = self.magnitude();

        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// projects the vector onto another vector
    pub fn proj(&self, other: &Vec3) -> Self {
        let a = self.dot(other);
        let b = other.dot(other);
        let c = a / b;

        Vec3 {
            x: other.x * c,
            y: other.y * c,
            z: other.z * c,
        }
    }

    /// computes the angle (in radians) between another vector
    pub fn angle(&self, other: &Vec3) -> f64 {
        let dot = self.dot(other);
        let mag_u = self.magnitude();
        let mag_v = other.magnitude();

        f64::acos(dot / mag_u / mag_v)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(vec: [f64; 3]) -> Self {
        let [x, y, z] = vec;
        Vec3 { x, y, z }
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
