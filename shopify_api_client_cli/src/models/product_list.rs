use super::product::Product;

#[derive(Debug, Clone)]
pub struct Product_List {
    items: Vec<Product>,
}

impl Product_List {
    pub fn new() -> Self {
        let list = Product_List {
            items: Vec::from([
                Product::new(1, "test 1".to_string(), 100, "test product 1".to_string()),
                Product::new(2, "test 2".to_string(), 200, "test product 2".to_string()),
                Product::new(3, "test 3".to_string(), 300, "test product 3".to_string()),
            ]),
        };

        return list;
    }

    pub fn items(&self) -> Vec<Product> {
        self.items.clone()
    }
}
