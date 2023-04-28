mod math;

#[cfg(test)]
pub mod tests {
  use super::math::tuple::Tuple;

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
    let result = x.add(&y);
    let expected = Tuple::new_tuple(1.0, 1.0, 6.0, 1.0);
    assert_eq!(result.equals(&expected), true);
  }
}