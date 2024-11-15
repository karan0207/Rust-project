use tch::{nn, Device, Tensor};
use tokenizers::{Tokenizer, PaddingParams, TruncationParams};


fn main() -> Result<(), Box<dyn std::error::Error>> {


// Load a pre-trained tokenizer (e.g., GPT-2)
let tokenizer = Tokenizer::from_pretrained("gpt2", None)?;

 // Set up the device (use GPU if available)
 let device = if tch::Cuda::is_available() {
    Device::Cuda(0)
} else {
    Device::Cpu
};

//  Load a pre-trained model (e.g., GPT-2)
    let vs = nn::VarStore::new(device);
    let model = tch::CModule::load_on_device("gpt2.pt", device)?;

      // Input sentence
      let input_text = "Once upon a time";

      // Tokenize the input
      let encoding = tokenizer.encode(input_text, true)?;
      let input_ids = Tensor::of_slice(&encoding.get_ids().iter().map(|&x| x as i64).collect::<Vec<_>>())
          .unsqueeze(0) // Add batch dimension
          .to(device);

     // Forward pass through the model
     let output = model.forward_ts(&[input_ids])?;
     let logits = output.get(0).unwrap(); // Get logits from the output
 
     // Decode output tokens
     let predicted_token_id = logits.argmax(2, true).int64_value(&[0, 0]);
     let predicted_token = tokenizer.decode(vec![predicted_token_id as u32], true)?;

}