use ndarray::Array2;

/// Load training data
pub fn load_training_data() -> (Array2<f64>, Array2<f64>) {
    let features = ndarray::array![[1.0], [2.0], [3.0], [4.0], [5.0]];
    let targets = ndarray::array![[2.0], [4.0], [6.0], [8.0], [10.0]];
    (features, targets)
}
