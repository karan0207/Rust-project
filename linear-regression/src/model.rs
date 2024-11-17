use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array2;

/// Train a linear regression model
pub fn train_model(features: &Array2<f64>, targets: &Array2<f64>) -> LinearRegression {
    let dataset = Dataset::new(features.clone(), targets.clone());
    LinearRegression::new()
        .fit(&dataset)
        .expect("Failed to fit the model")
}

/// Predict using the trained model
pub fn predict(model: &LinearRegression, inputs: &Array2<f64>) -> Array2<f64> {
    model.predict(inputs)
}
