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


    fn main() {
        // Example input
        println!("Enter customer name:");
        let mut customer_name = String::new();
        io::stdin().read_line(&mut customer_name).unwrap();
        let customer_name = customer_name.trim().to_string();
    
        println!("Enter invoice number:");
        let mut invoice_number = String::new();
        io::stdin().read_line(&mut invoice_number).unwrap();
        let invoice_number: u32 = invoice_number.trim().parse().unwrap();
    
        let mut invoice = Invoice::new(customer_name, invoice_number);
    
        loop {
            println!("Enter item description (or 'done' to finish):");
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            let description = description.trim().to_string();
    
            if description.to_lowercase() == "done" {
                break;
            }
    
            println!("Enter quantity:");
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).unwrap();
            let quantity: u32 = quantity.trim().parse().unwrap();
    
            println!("Enter unit price:");
            let mut unit_price = String::new();
            io::stdin().read_line(&mut unit_price).unwrap();
            let unit_price: f64 = unit_price.trim().parse().unwrap();
    
            invoice.add_item(description, quantity, unit_price);
        }
    
        println!("\nGenerated Invoice:\n");
        println!("{}", invoice.generate());
    
        let filename = format!("invoice_{}.txt", invoice.invoice_number);
        if let Err(e) = invoice.save_to_file(&filename) {
            eprintln!("Failed to save invoice: {}", e);
        } else {
            println!("Invoice saved to {}", filename);
        }
    }