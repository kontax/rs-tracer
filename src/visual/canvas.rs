use crate::visual::color::Color;

pub struct Canvas {
    w: usize,
    h: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let pixels = vec![Color::black(); w*h];
        Self { w, h, pixels }
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn h(&self) -> usize {
        self.h
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use crate::visual::color::Color;

    use super::*;

    #[test]
    fn pixels_in_a_canvas_are_initialised_black() {
        let c = Canvas::new(10, 20);
        let black = Color::black();

        assert_eq!(c.w(), 10);
        assert_eq!(c.h(), 20);
        for p in c.pixels {
            assert_eq!(p, black);
        }
    }
}
