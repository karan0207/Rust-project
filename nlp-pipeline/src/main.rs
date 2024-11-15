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

}