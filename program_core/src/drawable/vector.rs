use core::f64::EPSILON;
use core::ops::{Add, AddAssign, Sub, SubAssign};

/// # Vector2
/// structure to hold vectors in 2d cartesian space
/// this structure can perform all operations done on vectors
///
/// Vector2 instances can be added by + and +=, subtracted by - and -=.
///
/// # Examples
/// ```
/// use program_core::Vector2;
///
/// let v1 = Vector2::new(1.0, 1.0);
/// let v2 = Vector2::new(1.0, -1.0);
///
/// assert_eq!(v1.len(), v2.len());
/// assert_eq!(0.0, v1.dot(v2));
/// assert_eq!(2.0, v1.cross(v2).abs());
///
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    x: f64,
    y: f64,
    len: f64,
    arg: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {
            x,
            y,
            len: (x.powi(2) + y.powi(2)).sqrt(),
            arg: (y / x).atan(),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn len(&self) -> f64 {
        self.len
    }

    pub fn arg(&self) -> f64 {
        self.arg
    }

    /// ## Vector2::dot
    /// calculates the dot product between 2 vectors
    pub fn dot(&self, rhs: Vector2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// ## Vector2::cross
    /// calculates the cross product between 2 vectors
    pub fn cross(&self, rhs: Vector2) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// ## Vector2::translate
    /// shifts the vector by the given offset
    pub fn translate(&mut self, offset: Vector2) -> Self {
        self.x += offset.x;
        self.y += offset.y;

        self.len = (self.x.powi(2) + self.y.powi(2)).sqrt();
        self.arg = (self.y / self.x).atan();

        *self
    }

    /// ## Vector2::rotate
    /// rotates the vector by the given angle about the origin
    pub fn rotate(&mut self, angle: f64) -> Self {
        self.arg += angle;
        self.x = self.len * self.arg.cos();
        self.y = self.len * self.arg.sin();

        *self
    }

    /// ## Vector2::scale
    /// scales the length of the vector
    pub fn scale(&mut self, c: f64) -> Self {
        // scaling by zero will have no effect
        let c = if c == 0.0 { 1.0 } else { c };

        self.x *= c;
        self.y *= c;
        self.len *= c;

        *self
    }

    /// ## Vector2::equals_vector
    /// checks equality between another vector using differences
    /// use the == operator to use this
    fn equals_vector(&self, rhs: Vector2) -> bool {
        (self.x - rhs.x <= EPSILON) && (self.y - rhs.y <= EPSILON)
    }

    /// ##Vector2::equals_tuple
    /// checks equality between a vector and a tuple using differences
    /// use the == operator to use this.
    fn equals_tuple(&self, rhs: (f64, f64)) -> bool {
        self.equals_vector(Vector2::new(rhs.0, rhs.1))
    }
}

impl ToString for Vector2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vector2::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vector2::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.equals_vector(*other)
    }
}

impl PartialEq<(f64, f64)> for Vector2 {
    fn eq(&self, other: &(f64, f64)) -> bool {
        self.equals_tuple(*other)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn test_vector2() {
        let v = Vector2::new(1.0, 1.0);

        assert_eq!(2f64.sqrt(), v.len());
        assert_eq!(FRAC_PI_4, v.arg());
    }

    #[test]
    fn test_vector2_dot() {
        let v1 = Vector2::new(1.0, 1.0);
        let v2 = Vector2::new(2.0, 2.0);

        assert_eq!(4.0, v1.dot(v2));
        assert_eq!(v1.dot(v2), v2.dot(v1));

        let v1 = Vector2::new(0.0, 1.0);
        let v2 = Vector2::new(1.0, 0.0);

        assert_eq!(0.0, v1.dot(v2));
    }

    #[test]
    fn test_vector2_cross() {
        let v1 = Vector2::new(0.0, 1.0);
        let v2 = Vector2::new(1.0, 0.0);

        assert_eq!(-1.0, v1.cross(v2));
        assert_eq!(v1.cross(v2), -v2.cross(v1));

        let v2 = Vector2::new(0.0, 2.0);

        assert_eq!(0.0, v1.cross(v2));
    }

    #[test]
    fn test_vector2_translate() {
        let mut v1 = Vector2::new(0.0, 0.0);
        let offset = Vector2::new(1.0, 1.0);

        v1.translate(offset);

        assert_eq!(Vector2::new(1.0, 1.0), v1);
        assert_eq!(2f64.sqrt(), v1.len());
        assert_eq!(FRAC_PI_4, v1.arg());
    }

    #[test]
    fn test_vector2_rotate() {
        let mut v1 = Vector2::new(1.0, 1.0);

        v1.rotate(FRAC_PI_4);

        assert_eq!(Vector2::new(0.0, 2f64.sqrt()), v1);
        assert_eq!(2f64.sqrt(), v1.len());
        assert_eq!(FRAC_PI_2, v1.arg());
    }

    #[test]
    fn test_vector2_scale() {
        let mut v1 = Vector2::new(1.0, 1.0);

        v1.scale(2.0);

        assert_eq!(Vector2::new(2.0, 2.0), v1);
        assert_eq!(8f64.sqrt(), v1.len());
        assert_eq!(FRAC_PI_4, v1.arg());
    }
}
