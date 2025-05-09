// src/ml/matrix.rs

use ndarray::{Array, Array2, Axis, Ix2, LinalgScalar, ShapeError};
use ndarray::linalg::Dot; // For matrix multiplication
use serde::{Serialize, Deserialize}; // If ndarray's "serde" feature is enabled

/// A type alias for a 2D matrix of a generic type `A`.
/// Leverages `ndarray::Array2` for efficient operations.
/// The type `A` should typically be a numeric type like `f32`, `f64`, `i32`, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)] // Add Serialize, Deserialize
pub struct Matrix<A: LinalgScalar + Clone>(pub Array2<A>);


impl<A: LinalgScalar + Clone> Matrix<A> {
    /// Creates a new matrix from a nested Vec.
    /// Returns an error if the rows have inconsistent lengths.
    /// Example: `Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]])`
    pub fn from_vec(data: Vec<Vec<A>>) -> Result<Self, ShapeError> {
        if data.is_empty() {
            return Ok(Matrix(Array2::zeros((0, 0))));
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
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix(Array2::zeros((rows, cols)))
    }

    /// Creates a new matrix of ones with the given dimensions.
    pub fn ones(rows: usize, cols: usize) -> Self {
        Matrix(Array2::ones((rows, cols)))
    }
    
    /// Creates an identity matrix of size `n x n`.
    pub fn eye(n: usize) -> Self where A: num_traits::identities::One + num_traits::identities::Zero {
        Matrix(Array2::eye(n))
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
    /// Panics if matrices have incompatible dimensions for multiplication.
    pub fn dot<B, C>(&self, other: &Matrix<B>) -> Result<Matrix<C>, ShapeError>
    where
        A: Dot<B, Output = C>,
        B: LinalgScalar + Clone,
        C: LinalgScalar + Clone,
    {
        // self.0.dot(&other.0) checks dimensions internally and panics on mismatch.
        // To return a Result, we'd need to check dimensions beforehand.
        let (r1, c1) = self.dim();
        let (r2, c2) = other.dim();
        if c1 != r2 {
            return Err(ShapeError::from_kind(ndarray::ErrorKind::IncompatibleShape));
        }
        Ok(Matrix(self.0.dot(&other.0)))
    }
    
    /// Transposes the matrix.
    pub fn t(&self) -> Matrix<A> {
        Matrix(self.0.t().into_owned())
    }
    
    // Additional common operations can be added here:
    // - Scalar multiplication/division
    // - Element-wise multiplication/division
    // - Determinant, inverse (for square matrices)
    // - Slicing, joining, etc.
}


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2; // For concise array creation in tests

    #[test]
    fn test_matrix_from_vec() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let matrix = Matrix::from_vec(data).unwrap();
        assert_eq!(matrix.dim(), (2, 3));
        assert_eq!(matrix.0, arr2(&[[1, 2, 3], [4, 5, 6]]));
    }

    #[test]
    fn test_matrix_from_vec_empty() {
        let data: Vec<Vec<i32>> = vec![];
        let matrix = Matrix::from_vec(data).unwrap();
        assert_eq!(matrix.dim(), (0,0));
    }

    #[test]
    fn test_matrix_from_vec_shape_error() {
        let data = vec![vec![1, 2], vec![3, 4, 5]];
        let matrix_result = Matrix::from_vec(data);
        assert!(matrix_result.is_err());
    }

    #[test]
    fn test_matrix_zeros_ones_eye() {
        let zeros_matrix: Matrix<f64> = Matrix::zeros(2, 3);
        assert_eq!(zeros_matrix.0, Array2::zeros((2,3)));

        let ones_matrix: Matrix<i32> = Matrix::ones(3, 2);
        assert_eq!(ones_matrix.0, Array2::ones((3,2)));
        
        let eye_matrix: Matrix<f32> = Matrix::eye(3);
        assert_eq!(eye_matrix.0, Array2::eye(3));
    }

    #[test]
    fn test_matrix_get_set() {
        let mut matrix = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        assert_eq!(matrix.get(0, 1), Some(&2.0));
        assert_eq!(matrix.get(2, 0), None);
        
        matrix.set(1, 1, 5.5);
        assert_eq!(matrix.get(1, 1), Some(&5.5));
    }

    #[test]
    fn test_matrix_add() {
        let m1 = Matrix::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let m2 = Matrix::from_vec(vec![vec![5, 6], vec![7, 8]]).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.0, arr2(&[[6, 8], [10, 12]]));
    }
    
    #[test]
    fn test_matrix_add_shape_error() {
        let m1: Matrix<i32> = Matrix::zeros(2,2);
        let m2: Matrix<i32> = Matrix::zeros(2,3);
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_matrix_sub() {
        let m1 = Matrix::from_vec(vec![vec![5, 8], vec![3, 7]]).unwrap();
        let m2 = Matrix::from_vec(vec![vec![1, 2], vec![0, 4]]).unwrap();
        let result = m1.sub(&m2).unwrap();
        assert_eq!(result.0, arr2(&[[4, 6], [3, 3]]));
    }

    #[test]
    fn test_matrix_dot_product() {
        let m1 = Matrix::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap(); // 2x3
        let m2 = Matrix::from_vec(vec![vec![7, 8], vec![9, 10], vec![11, 12]]).unwrap(); // 3x2
        let result = m1.dot(&m2).unwrap(); // Should be 2x2
        // (1*7 + 2*9 + 3*11) = 7 + 18 + 33 = 58
        // (1*8 + 2*10 + 3*12) = 8 + 20 + 36 = 64
        // (4*7 + 5*9 + 6*11) = 28 + 45 + 66 = 139
        // (4*8 + 5*10 + 6*12) = 32 + 50 + 72 = 154
        assert_eq!(result.0, arr2(&[[58, 64], [139, 154]]));
    }
    
    #[test]
    fn test_matrix_dot_product_shape_error() {
        let m1: Matrix<i32> = Matrix::zeros(2,3);
        let m2: Matrix<i32> = Matrix::zeros(2,2); // Incompatible: 2x3 dot 2x2
        assert!(m1.dot(&m2).is_err());
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap(); // 2x3
        let mt = m.t(); // Should be 3x2
        assert_eq!(mt.dim(), (3, 2));
        assert_eq!(mt.0, arr2(&[[1, 4], [2, 5], [3, 6]]));
    }
    
    #[test]
    fn test_matrix_serialization_deserialization() {
        // This test requires ndarray's "serde" feature and Matrix to derive Serialize/Deserialize
        let matrix = Matrix::from_vec(vec![vec![1.1, 2.2], vec![3.3, 4.4]]).unwrap();
        let serialized = serde_json::to_string(&matrix).unwrap();
        let deserialized: Matrix<f64> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(matrix, deserialized);
    }
}
