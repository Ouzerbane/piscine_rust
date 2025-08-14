use std::ops::{Add, Mul};

#[derive(Debug, Clone,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        let mut x = 0;
        for i in &self.0 {
            if i.len() > x {
                x = i.len();
            }
        }
        x
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        for row in &self.0 {
            vec.push(row[n].clone());
        }
        vec
    }
}



impl< T: Copy + Mul<Output = T> + Add<Output = T> + Default> Mul for Matrix<T>
// where
//     T: Copy + Mul<Output = T> + Add<Output = T> + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_self = self.0.len();
        let cols_self = if rows_self > 0 { self.0[0].len() } else { 0 };
        let rows_rhs = rhs.0.len();
        let cols_rhs = if rows_rhs > 0 { rhs.0[0].len() } else { 0 };

        if cols_self != rows_rhs {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_rhs]; rows_self];

        for i in 0..rows_self {
            for j in 0..cols_rhs {
                let mut sum = T::default();
                for k in 0..cols_self {
                    sum = sum + (self.0[i][k] * rhs.0[k][j]);
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}