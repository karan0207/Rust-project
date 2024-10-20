// src/main.rs

use std::str::SplitWhitespace;

fn main() {
    // Example usage
    let expression = "+ 3 4";
    let result = calculate(expression);
    println!("Result of '{}': {}", expression, result);  // Output should be 7.0

    let expression = "- 3 * 4 5";
    let result = calculate(expression);
    println!("Result of '{}': {}", expression, result);  // Output should be -17.0
}

fn calculate(expression: &str) -> f64 {
    let mut tokens = expression.split_whitespace();
    evaluate(&mut tokens)
}

// Recursive evaluation function
fn evaluate(tokens: &mut SplitWhitespace) -> f64 {
    // Get the first token (operator or operand)
    let token = tokens.next().unwrap();

    // Check if the token is an operator
    match token {
        "+" => {
            let operand1 = evaluate(tokens);
            let operand2 = evaluate(tokens);
            operand1 + operand2
        },
        "-" => {
            let operand1 = evaluate(tokens);
            let operand2 = evaluate(tokens);
            operand1 - operand2
        },
        "*" => {
            let operand1 = evaluate(tokens);
            let operand2 = evaluate(tokens);
            operand1 * operand2
        },
        "/" => {
            let operand1 = evaluate(tokens);
            let operand2 = evaluate(tokens);
            operand1 / operand2
        },
        // If it's a number (operand), parse and return it
        _ => token.parse::<f64>().unwrap(),
    }
}
