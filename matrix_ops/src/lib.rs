use std::ops::{Add, Sub}; 
#[derive(Debug, Clone,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result: Vec<Vec<T>> = Vec::new();

        for (i, v) in self.0.iter().enumerate() {
            if other.0[i].len() != self.0[i].len() {
                return None;
            }
            let mut newarr: Vec<_> = vec![];
            for (x, y) in self.0[i].iter().zip(other.0[i].iter()) {
                newarr.push(*x + *y);
            }
            result.push(newarr);
            // for j in 0..
        }

        Some(Matrix(result))
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result: Vec<Vec<T>> = Vec::new();

        for (i, v) in self.0.iter().enumerate() {
            if other.0[i].len() != self.0[i].len() {
                return None;
            }
            let mut newarr: Vec<_> = vec![];
            for (x, y) in self.0[i].iter().zip(other.0[i].iter()) {
                newarr.push(*x - *y);
            }
            result.push(newarr);
            // for j in 0..
        }

        Some(Matrix(result))
    }
}
