use core::panic;

// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


// Define the `Order` struct
pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

impl Order {
    // Constructor with validation
    pub fn new(product_name: String, quantity: u16, unit_price: u16) -> Order {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot exceed 300 characters");
        }
        if quantity == 0 {
            panic!("Quantity cannot be zero");
        }
        if unit_price == 0 {
            panic!("Unit price cannot be zero");
        }

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    // Getter methods
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    // Compute total price
    pub fn total(&self) -> u32 {
        self.quantity as u32 * self.unit_price as u32
    }

    // Setter methods with validation
    pub fn set_product_name(&mut self, new_product: String) {
        if new_product.is_empty() {
            panic!("Product name cannot be empty");
        }
        if new_product.len() > 300 {
            panic!("Product name cannot exceed 300 characters");
        }
        self.product_name = new_product;
    }

    pub fn set_quantity(&mut self, new_quantity: u16) {
        if new_quantity == 0 {
            panic!("Quantity cannot be zero");
        }
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u16) {
        if new_unit_price == 0 {
            panic!("Unit price cannot be zero");
        }
        self.unit_price = new_unit_price;
    }
}
