use std::ops::Neg;

use super::{compare::equal, tuple::Tuple};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut data = vec![];

        for value in self.data {
            data.push(value * rhs);
        }

        Matrix {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert!(self.width == rhs.height);
        let mut data = vec![];

        for index in 0..rhs.width * rhs.height {
            let result_y = index / rhs.width;
            let result_x = index % rhs.width;

            let mut result_value = 0.0;
            for rhs_y in 0..rhs.height {
                let left = self.get(result_y, rhs_y);
                let right = rhs.get(rhs_y, result_x);
                result_value += left * right;
            }

            data.push(result_value);
        }

        Matrix {
            width: rhs.width,
            height: rhs.height,
            data,
        }
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let rhs_matrix = Matrix::create_matrix(1, 4, vec![rhs.x, rhs.y, rhs.z, rhs.w]);
        (self * &rhs_matrix).to_tuple()
    }
}

impl Matrix {
    pub fn create_matrix(width: usize, height: usize, data: Vec<f64>) -> Matrix {
        assert!(data.len() == width * height);
        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn create_identity(size: usize) -> Matrix {
        let mut data = vec![];
        for i in 0..size * size {
            let row = i / size;
            let col = i % size;
            if row == col {
                data.push(1.0);
            } else {
                data.push(0.0);
            }
        }
        Matrix {
            width: size,
            height: size,
            data,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> f64 {
        assert!(x < self.width && y < self.height);
        return self.data[self.width * y + x];
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for (index, value) in self.data.iter().enumerate() {
            if equal(other.data[index], *value) == false {
                return false;
            }
        }

        return true;
    }

    pub fn to_tuple(&self) -> Tuple {
        assert!((self.width == 4 && self.height == 1) || (self.width == 1 && self.height == 4));
        Tuple::new_tuple(self.data[0], self.data[1], self.data[2], self.data[3])
    }

    pub fn transpose(&self) -> Matrix {
        let mut data = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                data.push(self.get(y, x));
            }
        }

        Matrix {
            width: self.height,
            height: self.width,
            data,
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut data = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                if x == col || y == row {
                    continue;
                }

                data.push(self.get(y, x));
            }
        }

        Matrix {
            width: self.width - 1,
            height: self.height - 1,
            data,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            minor.neg()
        }
    }

    pub fn determinate_2x2(&self) -> f64 {
        assert!(self.width == 2 && self.height == 2);
        (self.get(0, 0) * self.get(1, 1)) - (self.get(0, 1) * self.get(1, 0))
    }

    pub fn determinant(&self) -> f64 {
        if self.width == 2 && self.height == 2 {
            return self.determinate_2x2();
        }

        let mut determinant = 0.0;

        for index in 0..self.width {
            let cofactor = self.cofactor(0, index);
            let data = self.data[index];
            determinant += data * cofactor;
        }

        return determinant;
    }

    pub fn is_invertable(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if self.is_invertable() == false {
            return None;
        }

        let mut data = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                data.push(self.cofactor(y, x));
            }
        }

        let result = Matrix::create_matrix(self.width, self.height, data).transpose()
            * (1.0 / self.determinant());

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*;

    #[test]
    fn create_matrix_2x2() {
        let matrix = Matrix::create_matrix(2, 2, vec![1.5, 2.5, 9.5, 4.5]);

        assert!(matrix.get(0, 0) == 1.5);
        assert!(matrix.get(0, 1) == 2.5);
        assert!(matrix.get(1, 0) == 9.5);
        assert!(matrix.get(1, 1) == 4.5);
    }

    #[test]
    fn create_matrix_3x3() {
        let matrix = Matrix::create_matrix(
            3,
            3,
            vec![-3.0, -2.0, -1.0, -4.0, -3.0, -2.0, 1.5, 2.5, 3.5],
        );

        assert!(matrix.get(0, 0) == -3.0);
        assert!(matrix.get(2, 0) == 1.5);
        assert!(matrix.get(0, 1) == -2.0);
        assert!(matrix.get(0, 2) == -1.0);
    }

    #[test]
    fn create_matrix_4x4() {
        let matrix = Matrix::create_matrix(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );

        assert!(matrix.get(0, 0) == 1.0);
        assert!(matrix.get(0, 3) == 4.0);
        assert!(matrix.get(1, 0) == 5.5);
        assert!(matrix.get(1, 2) == 7.5);
        assert!(matrix.get(2, 2) == 11.0);
        assert!(matrix.get(3, 0) == 13.5);
        assert!(matrix.get(3, 2) == 15.5);
    }

    #[test]
    fn matrix_equality() {
        let m1 = Matrix::create_matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::create_matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert!(m1.equals(&m2));
    }

    #[test]
    fn matrix_inequality() {
        let m1 = Matrix::create_matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::create_matrix(2, 2, vec![1.0, 2.0, 3.0, 4.5]);
        assert!(m1.equals(&m2) == false);
    }

    #[test]
    fn multiplication() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let m2 = Matrix::create_matrix(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let result = m1 * &m2;
        let expect = Matrix::create_matrix(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert!(result.equals(&expect));
    }

    #[test]
    fn multiplication_diff_size() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let m2 = Matrix::create_matrix(1, 4, vec![1.0, 2.0, 3.0, 1.0]);

        let result = m1 * &m2;
        let expect = Matrix::create_matrix(1, 4, vec![18.0, 24.0, 33.0, 1.0]);

        assert!(result.equals(&expect));
    }

    #[test]
    fn multiplication_tuple() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let t1 = Tuple::new_tuple(1.0, 2.0, 3.0, 1.0);

        let result = m1 * t1;
        let expect = Tuple::new_tuple(18.0, 24.0, 33.0, 1.0);

        assert!(result.equals(&expect));
    }

    #[test]
    fn create_identity() {
        let m1 = Matrix::create_identity(4);
        assert!(m1.get(0, 0) == 1.0);
        assert!(m1.get(1, 1) == 1.0);
        assert!(m1.get(2, 2) == 1.0);
        assert!(m1.get(3, 3) == 1.0);
    }

    #[test]
    fn identity_multiplication() {
        let m1 = Matrix::create_identity(4);
        let m2 = Matrix::create_matrix(
            4,
            4,
            vec![
                3.0, 4.0, 5.0, 6.0, 1.0, 2.0, 3.0, 4.0, 3.0, 4.0, 5.0, 6.0, 1.0, 2.0, 3.0, 4.0,
            ],
        );

        let result = m1 * &m2;
        assert!(result.equals(&m2));
    }

    #[test]
    fn transpose() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );

        let expect = Matrix::create_matrix(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );

        let result = m1.transpose();
        assert!(result.equals(&expect));
    }

    #[test]
    fn determinate_2x2() {
        let m1 = Matrix::create_matrix(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        let result = m1.determinate_2x2();
        let expect = 17.0;
        assert!(result == expect);
    }

    #[test]
    fn submatrix_1() {
        let m1 = Matrix::create_matrix(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        let result = m1.submatrix(0, 2);

        let expect = Matrix::create_matrix(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);

        assert!(result.equals(&expect));
    }

    #[test]
    fn submatrix_2() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );

        let result = m1.submatrix(2, 1);

        let expect =
            Matrix::create_matrix(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);

        assert!(result.equals(&expect));
    }

    #[test]
    fn calcualte_minor() {
        let result =
            Matrix::create_matrix(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0])
                .minor(1, 0);
        assert!(result == 25.0);
    }

    #[test]
    fn calculate_cofactor() {
        let m1 = Matrix::create_matrix(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let minor1 = m1.minor(0, 0);
        let cofactor1 = m1.cofactor(0, 0);
        let minor2 = m1.minor(1, 0);
        let cofactor2 = m1.cofactor(1, 0);
        assert!(minor1 == -12.0);
        assert!(cofactor1 == -12.0);
        assert!(minor2 == 25.0);
        assert!(cofactor2 == -25.0);
    }

    #[test]
    fn large_determinant_3x3() {
        let m1 = Matrix::create_matrix(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        let cofactor1 = m1.cofactor(0, 0);
        let cofactor2 = m1.cofactor(0, 1);
        let cofactor3 = m1.cofactor(0, 2);
        let determinant = m1.determinant();
        assert!(cofactor1 == 56.0);
        assert!(cofactor2 == 12.0);
        assert!(cofactor3 == -46.0);
        assert!(determinant == -196.0);
    }

    #[test]
    fn large_determinant_4x4() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );
        let cofactor1 = m1.cofactor(0, 0);
        let cofactor2 = m1.cofactor(0, 1);
        let cofactor3 = m1.cofactor(0, 2);
        let cofactor4 = m1.cofactor(0, 3);
        let determinant = m1.determinant();

        assert!(cofactor1 == 690.0);
        assert!(cofactor2 == 447.0);
        assert!(cofactor3 == 210.0);
        assert!(cofactor4 == 51.0);
        assert!(determinant == -4071.0);
    }

    #[test]
    fn is_invertable() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );
        assert!(m1.is_invertable() == true);
    }

    #[test]
    fn is_not_invertable() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );
        assert!(m1.is_invertable() == false);
    }

    #[test]
    fn inverse_1() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );

        let expected = Matrix::create_matrix(
            4,
            4,
            vec![
                -0.15385, -0.15385, -0.28205, -0.53845, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
            ],
        );

        let inverse = m1.inverse().expect("failed to inverse");

        assert!(inverse.equals(&expected) == true);
    }

    #[test]
    fn inverse_2() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
        );

        let expected = Matrix::create_matrix(
            4,
            4,
            vec![
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
            ],
        );

        let inverse = m1.inverse().expect("failed to inverse");

        assert!(inverse.equals(&expected) == true);
    }

    #[test]
    fn test_inverse() {
        let m1 = Matrix::create_matrix(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );

        let m2 = Matrix::create_matrix(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );

        let m3 = m1.clone() * &m2;

        let m4 = m3 * &m2.inverse().expect("m2 didn't get invsersed");

        assert!(m4.equals(&m1) == true);
    }
}
