use super::product::Product;

#[derive(Debug)]
pub struct Product_List {
    items: Vec<Product>,
}

impl Product_List {
    pub fn new() -> Self {
        let mut list = Product_List { items: Vec::new() };

        // TODO:
        // list.items.push(value);

        return list;
    }
}
