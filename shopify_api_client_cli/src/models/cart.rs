use std::collections::HashMap;

use super::product::Product;

#[derive(Debug)]
pub struct Cart {
    current_products: HashMap<i32, i32>,
}

impl Cart {
    pub fn new() -> Self {
        Cart {
            current_products: HashMap::<i32, i32>::new(),
        }
    }

    pub fn show_all(&self) -> HashMap<i32, i32> {
        self.current_products.clone()
    }

    pub fn add(&mut self, product: Product) {
        self.current_products
            .entry(product.id())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    pub fn remove(&mut self, product: Product) {
        self.current_products.remove(&product.id());
    }
}
