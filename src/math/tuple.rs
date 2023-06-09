use super::compare;

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

impl std::ops::MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
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
        Tuple {
          x, y, z, w: 0.0
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
          x, y, z, w: 1.0
        }
    }

    pub fn new_tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
          x, y, z, w
        }
    }

    pub fn is_vector(&self) -> bool {
        compare::equal(self.w, 0.0)
    }

    pub fn is_point(&self) -> bool {
        compare::equal(self.w, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
          self.x * self.x +
          self.y * self.y +
          self.z * self.z +
          self.w * self.w
        )
    }

    pub fn equals(&self, other: &Tuple) -> bool {
        compare::equal(self.x, other.x) &&
        compare::equal(self.y, other.y) &&
        compare::equal(self.z, other.z) &&
        compare::equal(self.w, other.w)
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
}
