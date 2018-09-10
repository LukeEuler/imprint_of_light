#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn grey(c: f64) -> Self {
        Self { r: c, g: c, b: c }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, s: f64) -> Color {
        Color {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }
}

impl std::iter::Sum for Color {
    fn sum<I>(iter: I) -> Color
    where
        I: Iterator<Item = Color>,
    {
        iter.fold(Color::black(), std::ops::Add::add)
    }
}
