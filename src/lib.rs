use std::{
    fmt::{Debug, Display},
    ops::Mul,
    sync::{Arc, Mutex},
    thread,
};

use rand::Rng;

#[derive(PartialEq)]
pub struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    pub fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn verify(data: &Vec<Vec<i32>>) -> bool {
        let first = data[0].len();

        for i in data {
            if i.len() != first {
                return false;
            }
        }

        true
    }

    fn can_multiply(a: &Matrix, b: &Matrix) -> bool {
        if a.0[0].len() != b.0.len() {
            return false;
        }

        true
    }

    fn max_string_len(&self) -> usize {
        let mut max = 0;

        for i in &self.0 {
            for k in i {
                if *k > max {
                    max = *k;
                }
            }
        }

        format!("{}", max).len()
    }

    pub fn new(data: Vec<Vec<i32>>) -> Result<Matrix, String> {
        if !Self::verify(&data) {
            return Err("Incorrect matrix data".to_string());
        }

        Ok(Matrix(data))
    }

    pub fn generate(a: usize, b: usize) -> Result<Matrix, String> {
        let mut matrix_data = Vec::new();

        let mut rng = rand::thread_rng();

        for i in 0..a {
            matrix_data.push(Vec::new());

            for _ in 0..b {
                matrix_data[i].push(rng.gen_range(0..25));
            }
        }

        Matrix::new(matrix_data)
    }

    pub fn multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
        if !Matrix::can_multiply(&a, &b) {
            return Err(format!("Cannot multiply {} and {}", a, b));
        }

        let mut new_matrix_data = Vec::new();

        let mut i = 0;

        while i < b.0[0].len() {
            let mut g = 0;

            while g < a.0.len() {
                let mut sum = 0;

                let mut k = 0;

                while k < b.0.len() {
                    if i == 0 && k == 0 {
                        new_matrix_data.push(Vec::new());
                    }

                    sum += a.0[g][k] * b.0[k][i];

                    k += 1;
                }

                new_matrix_data[g].push(sum);

                g += 1;
            }

            i += 1;
        }

        Matrix::new(new_matrix_data)
    }

    pub fn multiple_threads_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
        if !Matrix::can_multiply(&a, &b) {
            return Err(format!("Cannot multiply {} and {}", a, b));
        }

        let mut new_matrix_data: Vec<Vec<i32>> = Vec::new();

        for _ in 0..a.0.len() {
            new_matrix_data.push(Vec::new());
        }

        let mut handles = vec![];

        let threads_count = b.0[0].len();

        let new_matrix_data = Arc::new(Mutex::new(new_matrix_data));
        let a = Arc::new(Mutex::new(a.0.clone()));
        let b = Arc::new(Mutex::new(b.0.clone()));

        for h in 0..threads_count {
            let new_matrix_data = Arc::clone(&new_matrix_data);
            let a = Arc::clone(&a);
            let b = Arc::clone(&b);

            let handle = thread::spawn(move || {
                let mut new_matrix_data = new_matrix_data.lock().unwrap();
                let a = a.lock().unwrap();
                let b = b.lock().unwrap();

                let mut g = 0;

                while g < a.len() {
                    let mut sum = 0;

                    let mut k = 0;

                    while k < b.len() {
                        sum += a[g][k] * b[k][h];

                        k += 1;
                    }

                    new_matrix_data[g].push(sum);

                    g += 1;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Matrix::new(new_matrix_data.clone().lock().unwrap().to_owned())
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::multiple_threads_multiply(&self, &rhs).unwrap()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix({} * {})", &self.0.len(), &self.0[0].len())
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "\n".to_string();
        let max_len = self.max_string_len();

        for i in &self.0 {
            for k in i {
                let needed_spaces = max_len - format!("{}", k).len();
                res += &" ".repeat(needed_spaces);
                res += &format!("{} ", *k);
            }
            res += "\n";
        }

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn generate_matrix_test() {
        let a = Matrix::generate(2, 3).unwrap();
        assert!(a.size() == (2, 3));
    }

    #[test]
    fn new_matrix_test() {
        assert!(Matrix::new(vec![vec![1, 2, 3], vec![1, 2]]).is_err());
        assert!(Matrix::new(vec![vec![1, 2, 3], vec![1, 2, 3]]).is_ok());
    }

    #[test]
    fn multiply_matrix_tests() {
        let a = Matrix::new(vec![vec![1, 2, 2], vec![3, 1, 1]]).unwrap();
        let b = Matrix::new(vec![vec![4, 2], vec![3, 1], vec![1, 5]]).unwrap();

        let c = Matrix::new(vec![vec![12, 14], vec![16, 12]]).unwrap();
        let c1 = Matrix::multiply(&a, &b).unwrap();

        assert!(c1 == c);

        let a = Matrix::generate(2, 3).unwrap();
        let b = Matrix::generate(3, 2).unwrap();

        let c = a * b;

        assert!(c.size() == (2, 2));

        let a = Matrix::generate(5, 18).unwrap();
        let b = Matrix::generate(18, 9).unwrap();

        let c = a * b;

        assert!(c.size() == (5, 9));

        let a = Matrix::generate(5, 18).unwrap();
        let b = Matrix::generate(17, 9).unwrap();

        assert!(Matrix::multiple_threads_multiply(&a, &b).is_err());
    }
}
