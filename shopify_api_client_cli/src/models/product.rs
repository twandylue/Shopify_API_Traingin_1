#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    serial_number: u32,
    id: String,
    name: String,
    price: u32,
    description: String,
}

impl Product {
    pub fn new(id: String, name: String, price: u32, description: String) -> Self {
        Product {
            serial_number: 0,
            id,
            name,
            price,
            description,
        }
    }

    pub fn set_serial_number(&mut self, num: u32) {
        self.serial_number = num
    }

    pub fn serial_number(&self) -> u32 {
        self.serial_number
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
