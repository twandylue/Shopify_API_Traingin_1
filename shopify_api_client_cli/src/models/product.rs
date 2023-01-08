#[derive(Debug, PartialEq, Eq)]
pub struct Product {
    Id: i32,
}

impl Product {
    pub fn id(&self) -> i32 {
        self.Id
    }
}
