use crate::math::tuple::Tuple;

type Color = Tuple;

impl Color {
    pub fn new_color(red: f64, green: f64, blue: f64) -> Color {
        Color {
            x: red,
            y: green,
            z: blue,
            w: 0.0,
        }
    }

    pub fn red(&self) -> f64 {
        self.x
    }

    pub fn green(&self) -> f64 {
        self.y
    }

    pub fn blue(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        let x = Color::new_color(-0.5, 0.4, 1.7);
        assert_eq!(x.red(), -0.5);
        assert_eq!(x.green(), 0.4);
        assert_eq!(x.blue(), 1.7);
    }

    #[test]
    fn addition() {
        let a = Color::new_color(0.9, 0.6, 0.75);
        let b = Color::new_color(0.7, 0.1, 0.25);
        let result = a + b;
        let expect = Color::new_color(1.6, 0.7, 1.0);
        assert_eq!(result.equals(&expect), true);
    }

    #[test]
    fn subtraction() {
        let a = Color::new_color(0.9, 0.6, 0.75);
        let b = Color::new_color(0.7, 0.1, 0.25);
        let result = a - b;
        let expect = Color::new_color(0.2, 0.5, 0.5);
        assert_eq!(result.equals(&expect), true);
    }

    #[test]
    fn multiplication() {
        let a = Color::new_color(1.0, 0.2, 0.4);
        let b = Color::new_color(0.9, 1.0, 0.1);

        let result = a * b;
        let expect = Color::new_color(0.9, 0.2, 0.04);

        assert_eq!(result.equals(&expect), true);
    }

    #[test]
    fn scale() {
        let mut result = Color::new_color(0.2, 0.3, 0.4);
        result *= 2.0;
        let expect = Color::new_color(0.4, 0.6, 0.8);

        assert_eq!(result.equals(&expect), true);
    }
}
