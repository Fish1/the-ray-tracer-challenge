use std::ops::Neg;

use super::super::matrix::Matrix;

pub fn new_sheer(xpy: f64, xpz: f64, ypx: f64, ypz: f64, zpx: f64, zpy: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, xpy, xpz, 0.0, ypx, 1.0, ypz, 0.0, zpx, zpy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn new_translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn new_scale(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn new_rotation_x(rad: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            rad.cos(),
            rad.sin() * -1.0,
            0.0,
            0.0,
            rad.sin(),
            rad.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn new_rotation_y(rad: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            rad.cos(),
            0.0,
            rad.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            rad.sin().neg(),
            0.0,
            rad.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn new_rotation_z(rad: f64) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            rad.cos(),
            rad.sin().neg(),
            0.0,
            0.0,
            rad.sin(),
            rad.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::super::super::tuple::Tuple;
    use super::*;

    #[test]
    fn transform_translation() {
        let translation = new_translation(5.0, -3.0, 2.0);
        let point = Tuple::new_point(-3.0, 4.0, 5.0);

        let result = &translation * point;
        let expected = Tuple::new_point(2.0, 1.0, 7.0);

        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_reverse_translation() {
        let translation = new_translation(5.0, -3.0, 2.0);
        let translation_inverse = translation.inverse().expect("inverse did not work");
        let point = Tuple::new_point(-3.0, 4.0, 5.0);

        let result = &translation_inverse * point;
        let expected = Tuple::new_point(-8.0, 7.0, 3.0);

        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_vector_translation() {
        let translation = new_translation(5.0, -3.0, 2.0);
        let vector = Tuple::new_vector(-3.0, 4.0, 5.0);

        let result = &translation * vector;
        let expected = Tuple::new_vector(-3.0, 4.0, 5.0);

        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_scale() {
        let scale = new_scale(2.0, 3.0, 4.0);
        let point = Tuple::new_point(-4.0, 6.0, 8.0);
        let result = &scale * point;
        let expected = Tuple::new_point(-8.0, 18.0, 32.0);

        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_reverse_scale() {
        let scale = new_scale(2.0, 3.0, 4.0);
        let scale_inserse = scale.inverse().expect("inverse did not work");
        let point = Tuple::new_point(-4.0, 6.0, 8.0);

        let result = &scale_inserse * (&scale * point);
        let expected = Tuple::new_point(-4.0, 6.0, 8.0);

        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_vector_scale() {
        let scale = new_scale(2.0, 3.0, 4.0);
        let vector = Tuple::new_vector(-4.0, 6.0, 8.0);
        let result = &scale * vector;
        let expected = Tuple::new_vector(-8.0, 18.0, 32.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_scale_reflection() {
        let reflection = new_scale(-1.0, 1.0, 1.0);
        let point = Tuple::new_point(-4.0, 6.0, 8.0);
        let result = &reflection * point;
        let expected = Tuple::new_point(4.0, 6.0, 8.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_rotate_x_axis() {
        let rotation_half_quarter = new_rotation_x(PI / 4.0);
        let rotation_full_quarter = new_rotation_x(PI / 2.0);
        let point = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = &rotation_half_quarter * point;
        let expected_half_quarter =
            Tuple::new_point(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0);
        let full_quarter = &rotation_full_quarter * point;
        let expected_full_quarter = Tuple::new_point(0.0, 0.0, 1.0);

        assert!(half_quarter.equals(&expected_half_quarter));
        assert!(full_quarter.equals(&expected_full_quarter));
    }

    #[test]
    fn transform_rotate_y_axis() {
        let rotation_half_quarter = new_rotation_y(PI / 4.0);
        let rotation_full_quarter = new_rotation_y(PI / 2.0);
        let point = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = &rotation_half_quarter * point;
        let full_quarter = &rotation_full_quarter * point;
        let expected_half_quarter =
            Tuple::new_point((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0);
        let expected_full_quarter = Tuple::new_point(1.0, 0.0, 0.0);

        assert!(half_quarter.equals(&expected_half_quarter));
        assert!(full_quarter.equals(&expected_full_quarter));
    }

    #[test]
    fn transform_rotate_z_axis() {
        let rotation_half_quarter = new_rotation_z(PI / 4.0);
        let rotation_full_quarter = new_rotation_z(PI / 2.0);
        let point = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = &rotation_half_quarter * point;
        let full_quarter = &rotation_full_quarter * point;
        let expected_half_quarter = Tuple::new_point(
            (2.0 as f64).sqrt().neg() / 2.0,
            (2.0 as f64).sqrt() / 2.0,
            0.0,
        );
        let expected_full_quarter = Tuple::new_point(-1.0, 0.0, 0.0);

        assert!(half_quarter.equals(&expected_half_quarter));
        assert!(full_quarter.equals(&expected_full_quarter));
    }

    #[test]
    fn transform_shear_xpy() {
        let sheer = new_sheer(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(5.0, 3.0, 4.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_sheer_xpz() {
        let sheer = new_sheer(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(6.0, 3.0, 4.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_sheer_ypx() {
        let sheer = new_sheer(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(2.0, 5.0, 4.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_sheer_ypz() {
        let sheer = new_sheer(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(2.0, 7.0, 4.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_sheer_zpx() {
        let sheer = new_sheer(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(2.0, 3.0, 6.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_sheer_zpy() {
        let sheer = new_sheer(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        let result = &sheer * point;
        let expected = Tuple::new_point(2.0, 3.0, 7.0);
        assert!(result.equals(&expected));
    }

    #[test]
    fn transform_combo_1() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let rotation = new_rotation_x(PI / 2.0);
        let scale = new_scale(5.0, 5.0, 5.0);
        let translate = new_translation(10.0, 5.0, 7.0);

        let point2 = &rotation * point;
        let expected_point2 = Tuple::new_point(1.0, -1.0, 0.0);

        let point3 = &scale * point2;
        let expected_point3 = Tuple::new_point(5.0, -5.0, 0.0);

        let point4 = &translate * point3;
        let expected_point4 = Tuple::new_point(15.0, 0.0, 7.0);

        assert!(point2.equals(&expected_point2));
        assert!(point3.equals(&expected_point3));
        assert!(point4.equals(&expected_point4));
    }

    #[test]
    fn transform_combo_2() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let rotate = new_rotation_x(PI / 2.0);
        let scale = new_scale(5.0, 5.0, 5.0);
        let translate = new_translation(10.0, 5.0, 7.0);

        let full_transform = &translate * &(&scale * &rotate);

        let result = &full_transform * point;
        let expected = Tuple::new_point(15.0, 0.0, 7.0);

        assert!(result.equals(&expected));
    }
}
