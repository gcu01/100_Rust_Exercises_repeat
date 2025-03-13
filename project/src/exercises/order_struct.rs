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
mod private {
    pub struct Order {
        product_name: String,
        quantity: usize,
        unit_price: usize,
    }

    impl Order {
        pub fn default() -> Self {
            Order{product_name: "product".into(), quantity: 1, unit_price: 1}
        }
        pub fn new(&mut self, product_name: String, quantity: usize, unit_price: usize){
            if product_name.is_empty() {
                panic!("Empty Product Name");
            } else if product_name.len() > 300 {
                panic!("Product Name too long (over 300bytes)");
            } else if quantity == 0 {
                panic!("Quantity is empty");
            } else if unit_price == 0 {
                panic!("Unit Price is empty");
            }
            self.product_name = product_name;
            self.quantity = quantity;
            self.unit_price = unit_price;
            //Order{product_name, quantity, unit_price}
        }
        pub fn get_product_name(&self) -> &str {
            &self.product_name.as_str()
        }
        pub fn get_quantity(&self) -> usize {
            self.quantity
        }
        pub fn get_unit_price(&self) -> usize {
            self.unit_price
        }
        pub fn set_product_name(&mut self, product_name: String) {
            self.product_name = product_name;
        } 
        pub fn set_quantity(&mut self, quantity: usize) {
            self.quantity = quantity;
        }
        pub fn set_unit_price(&mut self, unit_price: usize) {
            self.unit_price = unit_price;
        }
        pub fn total(&self) -> usize {
            self.quantity * self.unit_price
        }

    }
}

#[cfg(test)]
mod tests {
    use super::private::*;

    #[test]
    #[should_panic]
    fn test1_new(){
        let mut p: Order = Order::default();//Order { product_name: "".into(), quantity: 0, unit_price: 0 };
        p.new("".into(), 27, 5);
    }
    #[test]
    fn test_getters_order() {
        let mut p: Order = Order::default();
        p.new("Product".into(), 4, 6);
        assert_eq!("Product", p.get_product_name());
        assert_eq!(4, p.get_quantity());
        assert_eq!(6, p.get_unit_price());
        //println!("{}", p.get_quantity());
    }
    #[test]
    fn test_setters() {
        let mut p: Order = Order::default();
        p.set_product_name("P1".into());
        p.set_quantity(3);
        p.set_unit_price(6);
        assert_eq!("P1", p.get_product_name());
        assert_eq!(3, p.get_quantity());
        assert_eq!(6, p.get_unit_price());
        assert_eq!(18, p.total());
    }
}