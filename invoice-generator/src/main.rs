use std::fs::File;
use std::io::{self, Write};
use std::fmt;

struct InvoiceItem {
    description: String,
    quantity: u32,
    unit_price: f64,
}

impl InvoiceItem {
    fn total(&self) -> f64 {
        self.quantity as f64 * self.unit_price
    }



    struct Invoice {
        items: Vec<InvoiceItem>,
        customer_name: String,
        invoice_number: u32,
    }
    
    impl Invoice {
        fn new(customer_name: String, invoice_number: u32) -> Self {
            Self {
                items: Vec::new(),
                customer_name,
                invoice_number,
            }
        }
    
        fn add_item(&mut self, description: String, quantity: u32, unit_price: f64) {
            let item = InvoiceItem {
                description,
                quantity,
                unit_price,
            };
            self.items.push(item);
        }
    
        fn total(&self) -> f64 {
            self.items.iter().map(|item| item.total()).sum()
        }
    
        fn generate(&self) -> String {
            let mut output = format!(
                "Invoice Number: {}\nCustomer: {}\n\nItems:\n",
                self.invoice_number, self.customer_name
            );
            output.push_str("Description           Quantity    Unit Price     Total\n");
            output.push_str("-------------------------------------------------------\n");
    
            for item in &self.items {
                output.push_str(&format!(
                    "{:<20} {:<10} {:<12} {:.2}\n",
                    item.description, item.quantity, item.unit_price, item.total()
                ));
            }
    
            output.push_str(&format!("\nTotal Amount: {:.2}\n", self.total()));
            output
        }
    
        fn save_to_file(&self, filename: &str) -> io::Result<()> {
            let mut file = File::create(filename)?;
            file.write_all(self.generate().as_bytes())?;
            Ok(())
        }
    }