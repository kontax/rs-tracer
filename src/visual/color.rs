use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r - other.r).abs() < f64::EPSILON &&
            (self.g - other.g).abs() < f64::EPSILON &&
            (self.b - other.b).abs() < f64::EPSILON
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b
        )
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs
        )
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors_gives_correct_new_color() {
        // Arrange
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let cmp = Color::new(1.6, 0.7, 1.0);

        // Act
        let res = c1 + c2;

        // Assert
        assert_eq!(cmp, res);
    }

    #[test]
    fn subtracting_colors_gives_correct_new_color() {
        // Arrange
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let cmp = Color::new(0.2, 0.5, 0.5);

        // Act
        let res = c1 - c2;

        // Assert
        assert_eq!(cmp, res);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar_gives_correct_new_color() {
        // Arrange
        let c1 = Color::new(0.2, 0.3, 0.4);
        let cmp = Color::new(0.4, 0.6, 0.8);

        // Act
        let res = c1 * 2.0;

        // Assert
        assert_eq!(cmp, res);
    }

    #[test]
    fn multiplying_colors_gives_correct_new_color() {
        // Arrange
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let cmp = Color::new(0.9, 0.2, 0.04);

        // Act
        let res = c1 * c2;

        // Assert
        assert_eq!(cmp, res);
    }
}
