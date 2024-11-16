use ndarray::Array2;

/// Print predictions in a user-friendly format
pub fn print_predictions(inputs: &Array2<f64>, predictions: &Array2<f64>) {
    println!("Predictions:");
    for (input, prediction) in inputs.genrows().into_iter().zip(predictions.genrows()) {
        println!("Input: {:?} => Prediction: {:?}", input, prediction);
    }
}
