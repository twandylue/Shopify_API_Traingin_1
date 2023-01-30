use super::product::Product;
use crate::client::graphql_client::GraphqlClient;

#[derive(Debug, Clone)]
pub struct ProductList {
    items: Vec<Product>,
}

impl ProductList {
    pub fn new() -> Self {
        ProductList { items: Vec::new() }
    }

    pub async fn dowload_products(&mut self) {
        // NOTE: API(query products)
        let client = GraphqlClient::new();
        let mut i = 1;
        match client.query_products(5).await {
            Ok(results) => {
                results.into_iter().for_each(|(id, title)| {
                    let mut product =
                        Product::new(id, title, 100, "default description".to_string());
                    product.set_serial_number(i);
                    i += 1;
                    self.items.push(product);
                });
            }
            Err(_) => panic!("query products is failed"),
        }
    }

    pub fn items(&self) -> Vec<Product> {
        self.items.clone()
    }
}
