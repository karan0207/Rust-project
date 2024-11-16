mod data;
mod model;
mod utils;

use data::load_training_data;
use model::{train_model, predict};
use utils::print_predictions;

fn main() {
    // Load training data
    let (features, targets) = load_training_data();

    // Train the model
    let fitted_model = train_model(&features, &targets);

    // Test predictions
    let test_features = ndarray::array![[6.0], [7.0]];
    let predictions = predict(&fitted_model, &test_features);

    // Display predictions
    print_predictions(&test_features, &predictions);
}