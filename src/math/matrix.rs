pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix() {
        let matrix = Matrix::create_matrix(
            4,
            4,
            vec![
                1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0,
            ],
        );
    }
}
