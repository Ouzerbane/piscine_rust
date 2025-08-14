pub use lalgebra_scalar::*;
#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
         let mut matri :Vec<Vec<_>> = vec![vec![];row]; 
        for i in 0..row {
            for j in 0..col {
               matri[i].push(<T as Scalar>::zero())  
            }
        }
        Matrix(matri)
        
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut matri :Vec<Vec<_>> = vec![vec![];n]; 
        for i in 0..n {
            for j in 0..n {
                if i == j {
                     matri[i].push(<T as Scalar>::one())
                }else{
                    matri[i].push(<T as Scalar>::zero())
                }
            }
        }
        Matrix(matri)
	}
}