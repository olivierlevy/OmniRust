// src/ml/mod.rs

pub mod matrix; // For matrix operations, likely wrapping ndarray
pub mod algorithms; // Placeholder for pre-built algorithms
pub mod integrations; // Placeholder for integrations like TensorFlow/ONNX

// Re-export key items from submodules
pub use matrix::Matrix;
pub use algorithms::LinearRegressionModel;
// When other algorithms are added, they can be re-exported here too.
// e.g., pub use algorithms::KMeansModel;
