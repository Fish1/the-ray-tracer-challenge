use super::compare;

pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
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

  pub fn add(&self, other: &Tuple) -> Tuple {
    Tuple {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w
    }
  }

  pub fn equals(&self, other: &Tuple) -> bool {
    compare::equal(self.x, other.x) &&
    compare::equal(self.y, other.y) &&
    compare::equal(self.z, other.z) &&
    compare::equal(self.w, other.w)
  }
}