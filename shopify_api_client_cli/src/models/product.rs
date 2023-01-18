#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    id: String,
    name: String,
    price: u32,
    description: String,
}

impl Product {
    pub fn new(id: String, name: String, price: u32, description: String) -> Self {
        Product {
            id,
            name,
            price,
            description,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn price(&self) -> u32 {
        self.price
    }
}
