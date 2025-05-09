// src/ml/algorithms.rs

//! # Pre-built Machine Learning Algorithms
//!
//! This module is a placeholder for implementations of common machine learning algorithms.
//! Developing robust, efficient, and general-purpose ML algorithms is a significant
//! undertaking.
//!
//! For production use and comprehensive algorithm suites, consider using dedicated
//! Rust ML crates such as:
//! - `linfa`: A crate that aims to provide a Rust equivalent of Python's `scikit-learn`.
//!   It offers various algorithms for classification, regression, clustering, etc.
//! - `smartcore`: Another comprehensive machine learning library in Rust, offering
//!   a wide range of algorithms and tools.
//!
//! ## Potential Algorithms to Include:
//!
//! ### Regression
//! - Linear Regression
//! - Polynomial Regression
//! - (Potentially) Support Vector Regression, Decision Tree Regression if building from scratch
//!
//! ### Classification
//! - Logistic Regression
//! - K-Nearest Neighbors (KNN)
//! - Naive Bayes
//! - (Potentially) Support Vector Machines (SVM), Decision Trees if building from scratch
//!
//! ### Clustering
//! - K-Means
//! - DBSCAN
//! - Hierarchical Clustering
//!
//! ### Dimensionality Reduction
//! - Principal Component Analysis (PCA)
//!
//! Each algorithm would typically involve:
//! - A struct to hold model parameters.
//! - A `fit` method to train the model on data.
//! - A `predict` method to make predictions on new data.
//! - Helper functions for data preprocessing, cross-validation, and metric evaluation.

pub mod linear_regression {
    use crate::ml::matrix::Matrix;
    use ndarray::{array, concatenate, s, Axis, Array, Array1, Array2, LinalgScalar, Ix2};
    use ndarray_linalg::{Solve, Inverse}; // For matrix inversion and solving linear systems
    use num_traits::{Float, Zero}; // Using f64 for simplicity

    /// Linear Regression model using the Normal Equation.
    #[derive(Debug)]
    pub struct LinearRegressionModel {
        /// The coefficients (weights) of the linear model.
        pub coefficients: Option<Array1<f64>>, // Storing as ndarray::Array1 for direct use with ndarray operations
        /// The intercept (bias) term of the linear model.
        pub intercept: Option<f64>,
    }

    impl LinearRegressionModel {
        /// Creates a new, untrained LinearRegressionModel.
        pub fn new() -> Self {
            LinearRegressionModel {
                coefficients: None,
                intercept: None,
            }
        }

        /// Fits the linear regression model to the provided training data.
        ///
        /// # Arguments
        /// * `features` - A `Matrix<f64>` where rows are samples and columns are features.
        /// * `targets` - A `Matrix<f64>` (single column) or `Array1<f64>` representing the target values.
        ///
        /// # Returns
        /// `Ok(())` if fitting was successful, `Err(String)` otherwise.
        pub fn fit(&mut self, features: &Matrix<f64>, targets: &Matrix<f64>) -> Result<(), String> {
            if features.0.nrows() != targets.0.nrows() {
                return Err("Number of samples in features and targets must match.".to_string());
            }
            if targets.0.ncols() != 1 {
                return Err("Targets matrix must have exactly one column.".to_string());
            }

            // Add a column of ones to features for the intercept term (X_b)
            let ones = Array2::ones((features.0.nrows(), 1));
            let x_b = concatenate![Axis(1), ones, features.0.clone()];
            
            // Normal Equation: theta = (X_b^T * X_b)^-1 * X_b^T * y
            let x_b_t = x_b.t();
            let x_b_t_x_b = x_b_t.dot(&x_b);
            
            let x_b_t_x_b_inv = match x_b_t_x_b.inv() {
                Ok(inv) => inv,
                Err(e) => return Err(format!("Failed to invert (X^T * X) matrix: {}. This can happen if features are linearly dependent.", e)),
            };

            let x_b_t_y = x_b_t.dot(&targets.0);
            let theta = x_b_t_x_b_inv.dot(&x_b_t_y); // This will be a column vector

            if theta.is_empty() {
                return Err("Calculated theta is empty.".to_string());
            }

            self.intercept = Some(theta[[0, 0]]); // First element is the intercept
            self.coefficients = Some(theta.slice(s![1.., 0]).to_owned()); // Rest are coefficients

            Ok(())
        }

        /// Predicts target values for new input features.
        ///
        /// # Arguments
        /// * `features` - A `Matrix<f64>` of input features for prediction.
        ///
        /// # Returns
        /// `Ok(Matrix<f64>)` containing predicted values (single column matrix),
        /// or `Err(String)` if the model is not trained or input dimensions are incorrect.
        pub fn predict(&self, features: &Matrix<f64>) -> Result<Matrix<f64>, String> {
            if self.coefficients.is_none() || self.intercept.is_none() {
                return Err("Model has not been trained. Call fit() first.".to_string());
            }

            let intercept = self.intercept.unwrap();
            let coefficients = self.coefficients.as_ref().unwrap();

            if features.0.ncols() != coefficients.len() {
                return Err(format!(
                    "Number of features in input ({}) does not match model coefficients ({}).",
                    features.0.ncols(),
                    coefficients.len()
                ));
            }

            // Add a column of ones for the intercept term
            let ones = Array2::ones((features.0.nrows(), 1));
            let x_b = concatenate![Axis(1), ones, features.0.clone()];
            
            // Reconstruct full theta (intercept + coefficients)
            let mut theta_vec = vec![intercept];
            theta_vec.extend_from_slice(coefficients.as_slice().unwrap());
            let theta_col_matrix = Array2::from_shape_vec((theta_vec.len(), 1), theta_vec)
                .map_err(|e| format!("Failed to reshape theta: {}", e))?;

            let predictions_arr = x_b.dot(&theta_col_matrix);
            Ok(Matrix(predictions_arr))
        }
    }

    impl Default for LinearRegressionModel {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linear_regression::LinearRegressionModel;
    use crate::ml::matrix::Matrix;
    use ndarray::array;

    const EPSILON: f64 = 1e-9; // Tolerance for float comparisons

    #[test]
    fn test_linear_regression_fit_predict_simple() {
        let mut model = LinearRegressionModel::new();

        // y = 2x + 1
        // Features: x values
        let features_vec = vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0], vec![5.0]];
        // Targets: y values
        let targets_vec = vec![vec![3.0], vec![5.0], vec![7.0], vec![9.0], vec![11.0]];
        
        let features = Matrix::from_vec(features_vec).unwrap();
        let targets = Matrix::from_vec(targets_vec).unwrap();

        let fit_result = model.fit(&features, &targets);
        assert!(fit_result.is_ok(), "Model fitting failed: {:?}", fit_result.err());

        assert!(model.intercept.is_some());
        assert!(model.coefficients.is_some());

        let intercept = model.intercept.unwrap();
        let coeffs = model.coefficients.as_ref().unwrap();

        // Check if intercept is close to 1.0
        assert!((intercept - 1.0).abs() < EPSILON, "Intercept: expected around 1.0, got {}", intercept);
        // Check if coefficient for x is close to 2.0
        assert_eq!(coeffs.len(), 1, "Expected 1 coefficient");
        assert!((coeffs[0] - 2.0).abs() < EPSILON, "Coefficient: expected around 2.0, got {}", coeffs[0]);

        // Test predictions
        let test_features_vec = vec![vec![6.0], vec![7.0]];
        let test_features = Matrix::from_vec(test_features_vec).unwrap();
        let predictions_result = model.predict(&test_features);
        assert!(predictions_result.is_ok(), "Prediction failed: {:?}", predictions_result.err());

        if let Ok(predictions) = predictions_result {
            assert_eq!(predictions.0.nrows(), 2);
            assert_eq!(predictions.0.ncols(), 1);
            // Expected: y = 2*6 + 1 = 13
            assert!((predictions.0[[0,0]] - 13.0).abs() < EPSILON, "Prediction for 6.0: expected 13.0, got {}", predictions.0[[0,0]]);
            // Expected: y = 2*7 + 1 = 15
            assert!((predictions.0[[1,0]] - 15.0).abs() < EPSILON, "Prediction for 7.0: expected 15.0, got {}", predictions.0[[1,0]]);
        }
    }

    #[test]
    fn test_linear_regression_fit_multiple_features() {
        let mut model = LinearRegressionModel::new();
        // y = 1*x1 + 2*x2 + 3
        let features_vec = vec![
            vec![1.0, 1.0], // y = 1*1 + 2*1 + 3 = 6
            vec![1.0, 2.0], // y = 1*1 + 2*2 + 3 = 8
            vec![2.0, 1.0], // y = 1*2 + 2*1 + 3 = 7
            vec![2.0, 3.0], // y = 1*2 + 2*3 + 3 = 11
        ];
        let targets_vec = vec![vec![6.0], vec![8.0], vec![7.0], vec![11.0]];

        let features = Matrix::from_vec(features_vec).unwrap();
        let targets = Matrix::from_vec(targets_vec).unwrap();

        let fit_result = model.fit(&features, &targets);
        assert!(fit_result.is_ok(), "Model fitting failed: {:?}", fit_result.err());
        
        let intercept = model.intercept.unwrap();
        let coeffs = model.coefficients.as_ref().unwrap();

        assert!((intercept - 3.0).abs() < EPSILON, "Intercept: expected ~3.0, got {}", intercept);
        assert_eq!(coeffs.len(), 2);
        assert!((coeffs[0] - 1.0).abs() < EPSILON, "Coeff1: expected ~1.0, got {}", coeffs[0]);
        assert!((coeffs[1] - 2.0).abs() < EPSILON, "Coeff2: expected ~2.0, got {}", coeffs[1]);

        // Test prediction
        let test_features_vec = vec![vec![3.0, 2.0]]; // Expected y = 1*3 + 2*2 + 3 = 10
        let test_features = Matrix::from_vec(test_features_vec).unwrap();
        let predictions_result = model.predict(&test_features);
        assert!(predictions_result.is_ok());
        if let Ok(predictions) = predictions_result {
            assert!((predictions.0[[0,0]] - 10.0).abs() < EPSILON, "Prediction: expected 10.0, got {}", predictions.0[[0,0]]);
        }
    }
}
// pub mod kmeans {
//     // ... K-Means clustering algorithm structure ...
// }
