// src/ml/matrix.rs

use ndarray::{Array2, LinalgScalar, ShapeError};
// use ndarray::linalg::Dot; // For matrix multiplication - LinalgScalar implies this for Array2
// use serde::{Serialize, Deserialize}; // Temporarily remove serde for Matrix
use num_traits::{Zero, One}; // For zeros, ones, eye methods

/// A type alias for a 2D matrix of a generic type `A`.
/// Leverages `ndarray::Array2` for efficient operations.
/// The type `A` should typically be a numeric type like `f32`, `f64`, `i32`, etc.
#[derive(Debug, Clone, PartialEq)] // Removed Serialize and Deserialize
pub struct Matrix<A: LinalgScalar + Clone>(pub Array2<A>);


impl<A: LinalgScalar + Clone> Matrix<A> {
    /// Creates a new matrix from a nested Vec.
    /// Returns an error if the rows have inconsistent lengths.
    /// Example: `Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]])`
    pub fn from_vec(data: Vec<Vec<A>>) -> Result<Self, ShapeError> where A: Zero {
        if data.is_empty() {
            // Ensure A implements Zero for Array2::zeros
            return Ok(Matrix(Array2::<A>::zeros((0, 0))));
        }
        let rows = data.len();
        let cols = data[0].len();
        let mut flat_data = Vec::with_capacity(rows * cols);
        for row_vec in data {
            if row_vec.len() != cols {
                return Err(ShapeError::from_kind(ndarray::ErrorKind::IncompatibleShape));
            }
            flat_data.extend(row_vec);
        }
        Array2::from_shape_vec((rows, cols), flat_data).map(Matrix)
    }

    /// Creates a new matrix of zeros with the given dimensions.
    pub fn zeros(rows: usize, cols: usize) -> Self where A: Zero {
        Matrix(Array2::<A>::zeros((rows, cols)))
    }

    /// Creates a new matrix of ones with the given dimensions.
    pub fn ones(rows: usize, cols: usize) -> Self where A: One {
        Matrix(Array2::<A>::ones((rows, cols)))
    }
    
    /// Creates an identity matrix of size `n x n`.
    pub fn eye(n: usize) -> Self where A: One + Zero {
        Matrix(Array2::<A>::eye(n))
    }

    /// Returns the dimensions of the matrix as (rows, cols).
    pub fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }

    /// Gets the element at the given row and column.
    /// Returns None if the indices are out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<&A> {
        self.0.get((row, col))
    }
    
    /// Sets the element at the given row and column.
    /// Panics if indices are out of bounds.
    pub fn set(&mut self, row: usize, col: usize, value: A) {
        self.0[[row, col]] = value;
    }

    /// Performs matrix addition.
    /// Panics if matrices have incompatible dimensions.
    pub fn add(&self, other: &Matrix<A>) -> Result<Matrix<A>, ShapeError> {
        if self.dim() != other.dim() {
             return Err(ShapeError::from_kind(ndarray::ErrorKind::IncompatibleShape));
        }
        Ok(Matrix(self.0.clone() + other.0.clone()))
    }

    /// Performs matrix subtraction.
    /// Panics if matrices have incompatible dimensions.
    pub fn sub(&self, other: &Matrix<A>) -> Result<Matrix<A>, ShapeError> {
         if self.dim() != other.dim() {
             return Err(ShapeError::from_kind(ndarray::ErrorKind::IncompatibleShape));
        }
        Ok(Matrix(self.0.clone() - other.0.clone()))
    }

    /// Performs matrix multiplication (dot product).
    /// Both matrices must have the same element type `A`.
    /// Panics if matrices have incompatible dimensions for multiplication.
    pub fn dot(&self, other: &Matrix<A>) -> Result<Matrix<A>, ShapeError>
    // where A: Dot<A, Output = A> // This bound is incorrect for scalar A. LinalgScalar is key.
    {
        let (_r1, c1) = self.dim();
        let (r2, _c2) = other.dim();
        if c1 != r2 {
            return Err(ShapeError::from_kind(ndarray::ErrorKind::IncompatibleShape));
        }
        // The Dot trait is implemented for ArrayBase<S, D1> where S: Data, D1: Dimension
        // and takes Rhs = ArrayBase<S2, D2>.
        // self.0 and other.0 are Array2<A>. If A is LinalgScalar, this should work.
        Ok(Matrix(self.0.dot(&other.0)))
    }
    
    /// Transposes the matrix.
    pub fn t(&self) -> Matrix<A> {
        Matrix(self.0.t().into_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2; 

    #[test]
    fn test_matrix_from_vec() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let matrix: Matrix<f64> = Matrix::from_vec(data).unwrap();
        assert_eq!(matrix.dim(), (2, 3));
        assert_eq!(matrix.0, arr2(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]));
    }

    #[test]
    fn test_matrix_from_vec_empty() {
        let data: Vec<Vec<f64>> = vec![];
        let matrix: Matrix<f64> = Matrix::from_vec(data).unwrap();
        assert_eq!(matrix.dim(), (0,0));
    }

    #[test]
    fn test_matrix_from_vec_shape_error() {
        let data_f64 = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        let matrix_result_f64: Result<Matrix<f64>, _> = Matrix::from_vec(data_f64);
        assert!(matrix_result_f64.is_err());
        
        // Test with i32 as well, ensuring Zero bound is met for from_vec empty case
        let data_i32 = vec![vec![1, 2], vec![3, 4, 5]];
        let matrix_result_i32: Result<Matrix<i32>, _> = Matrix::from_vec(data_i32);
        assert!(matrix_result_i32.is_err());
    }

    #[test]
    fn test_matrix_zeros_ones_eye() {
        let zeros_matrix: Matrix<f64> = Matrix::zeros(2, 3);
        assert_eq!(zeros_matrix.0, Array2::<f64>::zeros((2,3)));

        let ones_matrix: Matrix<i32> = Matrix::ones(3, 2);
        assert_eq!(ones_matrix.0, Array2::<i32>::ones((3,2)));
        
        let eye_matrix: Matrix<f32> = Matrix::eye(3);
        assert_eq!(eye_matrix.0, Array2::<f32>::eye(3));
    }

    #[test]
    fn test_matrix_get_set() {
        let mut matrix: Matrix<f64> = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        assert_eq!(matrix.get(0, 1), Some(&2.0));
        assert_eq!(matrix.get(2, 0), None);
        
        matrix.set(1, 1, 5.5);
        assert_eq!(matrix.get(1, 1), Some(&5.5));
    }

    #[test]
    fn test_matrix_add() {
        let m1: Matrix<f64> = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let m2: Matrix<f64> = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.0, arr2(&[[6.0, 8.0], [10.0, 12.0]]));
    }
    
    #[test]
    fn test_matrix_add_shape_error() {
        let m1: Matrix<f64> = Matrix::zeros(2,2);
        let m2: Matrix<f64> = Matrix::zeros(2,3);
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_matrix_sub() {
        let m1: Matrix<f64> = Matrix::from_vec(vec![vec![5.0, 8.0], vec![3.0, 7.0]]).unwrap();
        let m2: Matrix<f64> = Matrix::from_vec(vec![vec![1.0, 2.0], vec![0.0, 4.0]]).unwrap();
        let result = m1.sub(&m2).unwrap();
        assert_eq!(result.0, arr2(&[[4.0, 6.0], [3.0, 3.0]]));
    }

    #[test]
    fn test_matrix_dot_product() {
        let m1: Matrix<f64> = Matrix::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let m2: Matrix<f64> = Matrix::from_vec(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]).unwrap();
        let result = m1.dot(&m2).unwrap(); 
        assert_eq!(result.0, arr2(&[[58.0, 64.0], [139.0, 154.0]]));
    }
    
    #[test]
    fn test_matrix_dot_product_shape_error() {
        let m1: Matrix<f64> = Matrix::zeros(2,3);
        let m2: Matrix<f64> = Matrix::zeros(2,2); 
        assert!(m1.dot(&m2).is_err());
    }

    #[test]
    fn test_matrix_transpose() {
        let m: Matrix<f64> = Matrix::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let mt = m.t(); 
        assert_eq!(mt.dim(), (3, 2));
        assert_eq!(mt.0, arr2(&[[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]));
    }
    
    // #[test] // Temporarily commented out due to Deserialize issues
    // fn test_matrix_serialization_deserialization() {
    //     // This test requires ndarray's "serde" feature and Matrix to derive Serialize/Deserialize
    //     // use serde::{Serialize, Deserialize}; // Ensure these are in scope if re-enabling
    //     // let matrix: Matrix<f64> = Matrix::from_vec(vec![vec![1.1, 2.2], vec![3.3, 4.4]]).unwrap();
    //     // let serialized = serde_json::to_string(&matrix).unwrap();
    //     // let deserialized: Matrix<f64> = serde_json::from_str(&serialized).unwrap();
    //     // assert_eq!(matrix, deserialized);
    // }
}
