pub struct PersonBuilder {
    name: Option<String>,
    age: Option<u8>,
    address: Option<String>,
}

impl PersonBuilder {
    pub fn new() -> Self {
        PersonBuilder {
            name: None,
            age: None,
            address: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_string());
        self
    }

    pub fn build(self) -> Person {
        Person {
            name: self.name.expect("Name is required"),
            age: self.age.expect("Age is required"),
            address: self.address.expect("Address is required"),
        }
    }
}

impl Default for PersonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Person {
    pub name: String,
    pub age: u8,
    pub address: String,
}
