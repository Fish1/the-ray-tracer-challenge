#[derive(Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        assert!(self.width == rhs.height);
        let mut data = vec![];

        for index in 0..rhs.width * rhs.height {
            let y = index / rhs.height;
            let x = index % rhs.width;

            let mut result = 0.0;
            for index2 in 0..rhs.height {
                println!("a {} {}", y, index2);
                let left = self.get(y, index2);
                println!("b {} {}", index2, x);
                println!("{:?}", rhs);
                let right = rhs.get(index2, x);
                result += left * right;
            }

            data.push(result);
        }

        Matrix {
            width: rhs.width,
            height: rhs.height,
            data,
        }
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

    pub fn get(&self, y: usize, x: usize) -> f64 {
        return self.data[self.height * y + x];
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for (index, value) in self.data.iter().enumerate() {
            if other.data[index] != *value {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
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

        let result = m1 * m2;
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

        let result = m1 * m2;
        println!("{:?}", result);

        let expect = Matrix::create_matrix(1, 4, vec![18.0, 24.0, 33.0, 1.0]);

        assert!(result.equals(&expect));
    }
}
