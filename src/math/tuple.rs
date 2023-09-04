use super::compare;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl std::ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::AddAssign<Tuple> for Tuple {
    fn add_assign(&mut self, rhs: Tuple) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl std::ops::Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Mul for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl std::ops::MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl std::ops::MulAssign<Tuple> for Tuple {
    fn mul_assign(&mut self, rhs: Tuple) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl std::ops::DivAssign<f64> for Tuple {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            w: self.w * -1.0,
        }
    }
}

impl Tuple {
    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        compare::equal(self.w, 0.0)
    }

    pub fn is_point(&self) -> bool {
        compare::equal(self.w, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn equals(&self, other: &Tuple) -> bool {
        compare::equal(self.x, other.x)
            && compare::equal(self.y, other.y)
            && compare::equal(self.z, other.z)
            && compare::equal(self.w, other.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_point() {
        let x = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(x.x, 4.3);
        assert_eq!(x.y, -4.2);
        assert_eq!(x.z, 3.1);
        assert_eq!(x.is_vector(), false);
        assert_eq!(x.is_point(), true);
    }

    #[test]
    fn tuple_vector() {
        let x = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(x.x, 4.3);
        assert_eq!(x.y, -4.2);
        assert_eq!(x.z, 3.1);
        assert_eq!(x.is_vector(), true);
        assert_eq!(x.is_point(), false);
    }

    #[test]
    fn compare_tuples_equal() {
        let x = Tuple::new_point(5.1, 3.4, -1.4);
        let y = Tuple::new_point(5.1, 3.4, -1.4);
        assert_eq!(x.equals(&y), true);
    }

    #[test]
    fn compare_tuples_not_equal() {
        let x = Tuple::new_point(5.2, 3.4, -1.4);
        let y = Tuple::new_point(5.1, 3.4, -1.4);
        assert_eq!(x.equals(&y), false);
    }

    #[test]
    fn add_tuples() {
        let x = Tuple::new_vector(3.0, -2.0, 5.0);
        let y = Tuple::new_point(-2.0, 3.0, 1.0);
        let result = x + y;
        let expected = Tuple::new_tuple(1.0, 1.0, 6.0, 1.0);
        assert_eq!(result.equals(&expected), true);
    }

    #[test]
    fn point_minus_vector() {
        let x = Tuple::new_point(3.0, 2.0, 1.0);
        let y = Tuple::new_vector(5.0, 6.0, 7.0);
        let result = x - y;
        let expected = Tuple::new_point(-2.0, -4.0, -6.0);
        assert_eq!(result.equals(&expected), true);
    }

    #[test]
    fn vector_minus_vector() {
        let x = Tuple::new_vector(3.0, 2.0, 1.0);
        let y = Tuple::new_vector(5.0, 6.0, 7.0);
        let result = x - y;
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert_eq!(result.equals(&expected), true);
    }

    #[test]
    fn negate_tuple() {
        let x = -Tuple::new_tuple(1.0, -2.0, 3.0, -4.0);
        let expect = Tuple::new_tuple(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(x.equals(&expect), true);
    }

    #[test]
    fn scale_tuple() {
        let mut x = Tuple::new_tuple(1.0, -2.0, 3.0, -4.0);
        x *= 3.5;
        let expect = Tuple::new_tuple(3.5, -7.0, 10.5, -14.0);
        assert_eq!(x.equals(&expect), true);
    }

    #[test]
    fn divide_tuple() {
        let mut x = Tuple::new_tuple(1.0, -2.0, 3.0, -4.0);
        x /= 2.0;
        let expect = Tuple::new_tuple(0.5, -1.0, 1.5, -2.0);
        assert_eq!(x.equals(&expect), true);
    }

    #[test]
    fn magnitude_1() {
        let x = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(x.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_2() {
        let x = Tuple::new_vector(0.0, 1.0, 0.0);
        assert_eq!(x.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_3() {
        let x = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(x.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_4() {
        let x = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(x.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn magnitude_5() {
        let x = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(x.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize_1() {
        let x = Tuple::new_vector(4.0, 0.0, 0.0);
        let expect = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(x.normalize().equals(&expect), true);
    }

    #[test]
    fn normalize_2() {
        let x = Tuple::new_vector(1.0, 2.0, 3.0);
        let z = f64::sqrt(14.0);
        let expect = Tuple::new_vector(1.0 / z, 2.0 / z, 3.0 / z);
        assert_eq!(x.normalize().equals(&expect), true);
    }

    #[test]
    fn normalize_3() {
        let x = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(x.normalize().magnitude() - 1.0 < f64::EPSILON, true);
    }

    #[test]
    fn dot_1() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        let result = a.dot(&b);

        assert_eq!(compare::equal(result, 20.0), true);
    }

    #[test]
    fn cross_1() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        let ab_result = a.cross(&b);
        let ab_expect = Tuple::new_vector(-1.0, 2.0, -1.0);

        assert_eq!(ab_result.equals(&ab_expect), true);
    }

    #[test]
    fn cross_2() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        let ba_result = b.cross(&a);
        let ba_expect = Tuple::new_vector(1.0, -2.0, 1.0);

        assert_eq!(ba_result.equals(&ba_expect), true);
    }
}
