#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    id: u32,
    name: String,
    price: u32,
    description: String,
}

impl Product {
    pub fn new(id: u32, name: String, price: u32, description: String) -> Self {
        Product {
            id,
            name,
            price,
            description,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
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
