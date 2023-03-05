use std::ops::{Add, Div, Mul, Neg, Sub, BitXor};

pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;

    fn zero() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    fn z(&self) -> f64 { self.z }
    fn w(&self) -> f64 { 1.0 }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON &&
            (self.y - other.y).abs() < f64::EPSILON &&
            (self.z - other.z).abs() < f64::EPSILON
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Sub<Vector> for Point {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x, y: -self.y, z: -self.z
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalise(&self) -> Self {
        *self / self.magnitude()
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    fn z(&self) -> f64 { self.z }
    fn w(&self) -> f64 { 0.0 }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON &&
            (self.y - other.y).abs() < f64::EPSILON &&
            (self.z - other.z).abs() < f64::EPSILON
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x, y: -self.y, z: -self.z
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl BitXor for Vector {
    type Output = f64;

    /// "Dot" product for `Vector`'s
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.x * rhs.x
            + self.y * rhs.y
            + self.z * rhs.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        // Arrange
        let p = Point::new(4.3, -4.2, 3.1);

        // Assert
        assert!(p.w() == 1.0);
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        // Arrange
        let t = Vector::new(4.3, -4.2, 3.1);

        // Assert
        assert!(t.w() == 0.0);
    }

    #[test]
    fn point_equals_same_point() {
        // Arrange
        let p1 = Point::new(4.3, -4.2, 3.1);
        let p2 = Point::new(4.3, -4.2, 3.1);

        // Assert
        assert_eq!(p1, p2);
    }

    #[test]
    fn vector_equals_same_vector() {
        // Arrange
        let v1 = Vector::new(4.3, -4.2, 3.1);
        let v2 = Vector::new(4.3, -4.2, 3.1);

        // Assert
        assert_eq!(v1, v2);
    }

    #[test]
    fn point_doesnt_equal_different_point() {
        // Arrange
        let p1 = Point::new(4.3, -4.2, 3.1);
        let p2 = Point::new(3.1, 0.8, 3.1);

        // Assert
        assert_ne!(p1, p2);
    }

    #[test]
    fn vector_doesnt_equal_different_vector() {
        // Arrange
        let v1 = Vector::new(4.3, -4.2, 3.1);
        let v2 = Vector::new(3.1, 0.8, 3.1);

        // Assert
        assert_ne!(v1, v2);
    }

    #[test]
    fn adding_vectors_to_points_gives_correct_point() {
        // Arrange
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let cmp = Point::new(1.0, 1.0, 6.0);

        // Act
        let result = v + p;

        // Assert
        assert_eq!(result, cmp);
    }

    #[test]
    fn adding_vectors_to_vectors_gives_correct_vector() {
        // Arrange
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let cmp = Vector::new(1.0, 1.0, 6.0);

        // Act
        let result = v1 + v2;

        // Assert
        assert_eq!(result, cmp);
    }

    #[test]
    fn subtracting_two_points_gives_correct_vector() {
        // Arrange
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let cmp = Vector::new(-2.0, -4.0, -6.0);

        // Act
        let result = p1 - p2;

        // Assert
        assert_eq!(result, cmp);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_gives_a_point() {
        // Arrange
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let cmp = Vector::new(-2.0, -4.0, -6.0);

        // Act
        let result = p - v;

        // Assert
        assert_eq!(result, cmp);
    }

    #[test]
    fn subtracting_two_vectors_gives_correct_vector() {
        // Arrange
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let cmp = Vector::new(-2.0, -4.0, -6.0);

        // Act
        let result = v1 - v2;

        // Assert
        assert_eq!(result, cmp);
    }

    #[test]
    fn negating_a_vector_works() {
        // Arrange
        let v = Vector::new(1.0, -2.0, 3.0);
        let cmp = Vector::new(-1.0, 2.0, -3.0);

        // Assert
        assert_eq!(-v, cmp);
    }

    #[test]
    fn negating_a_point_works() {
        // Arrange
        let p = Point::new(1.0, -2.0, 3.0);
        let cmp = Point::new(-1.0, 2.0, -3.0);

        // Assert
        assert_eq!(-p, cmp);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar_give_correct_vector() {
        // Arrange
        let v = Vector::new(1.0, -2.0, 3.0);
        let cmp = Vector::new(3.5, -7.0, 10.5);

        // Act
        let res = v * 3.5;

        // Assert
        assert_eq!(res, cmp);
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction_give_correct_vector() {
        // Arrange
        let v = Vector::new(1.0, -2.0, 3.0);
        let cmp = Vector::new(0.5, -1.0, 1.5);

        // Act
        let res = v * 0.5;

        // Assert
        assert_eq!(res, cmp);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar_gives_correct_vector() {
        // Arrange
        let v = Vector::new(1.0, -2.0, 3.0);
        let cmp = Vector::new(0.5, -1.0, 1.5);

        // Act
        let res = v / 2.0;

        // Assert
        assert_eq!(res, cmp);
    }

    #[test]
    fn magnitude_of_vector_x_1() {
        // Arrange
        let v = Vector::new(1.0, 0.0, 0.0);

        // Act
        let res = v.magnitude();

        // Assert
        assert_eq!(res, 1.0);
    }

    #[test]
    fn magnitude_of_vector_y_1() {
        // Arrange
        let v = Vector::new(0.0, 1.0, 0.0);

        // Act
        let res = v.magnitude();

        // Assert
        assert_eq!(res, 1.0);
    }

    #[test]
    fn magnitude_of_vector_z_1() {
        // Arrange
        let v = Vector::new(0.0, 0.0, 1.0);

        // Act
        let res = v.magnitude();

        // Assert
        assert_eq!(res, 1.0);
    }

    #[test]
    fn magnitude_of_positive_vector() {
        // Arrange
        let v = Vector::new(1.0, 2.0, 3.0);

        // Act
        let res = v.magnitude();

        // Assert
        assert_eq!(res, 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_of_negative_vector() {
        // Arrange
        let v = Vector::new(-1.0, -2.0, -3.0);

        // Act
        let res = v.magnitude();

        // Assert
        assert_eq!(res, 14.0_f64.sqrt());
    }

    #[test]
    fn normalisation_of_x_vector() {
        // Arrange
        let v = Vector::new(4.0, 0.0, 0.0);
        let cmp = Vector::new(1.0, 0.0, 0.0);

        // Act
        let res = v.normalise();

        // Assert
        assert_eq!(res, cmp);
    }

    #[test]
    fn normalisation_of_vector() {
        // Arrange
        let v = Vector::new(1.0, 2.0, 3.0);
        let cmp = Vector::new(
            1.0/f64::sqrt(14.0),
            2.0/f64::sqrt(14.0),
            3.0/f64::sqrt(14.0),
        );

        // Act
        let res = v.normalise();

        // Assert
        assert_eq!(res, cmp);
    }

    #[test]
    fn dot_product_of_vector() {
        // Arrange
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        // Act
        let res = v1 ^ v2;

        // Assert
        assert_eq!(res, 20.0);
    }

    #[test]
    fn cross_product_of_vector() {
        // Arrange
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let cmp1 = Vector::new(-1.0, 2.0, -1.0);
        let cmp2 = Vector::new(1.0, -2.0, 1.0);

        // Act
        let res1 = v1 * v2;
        let res2 = v2 * v1;

        // Assert
        assert_eq!(res1, cmp1);
        assert_eq!(res2, cmp2);
    }
}
