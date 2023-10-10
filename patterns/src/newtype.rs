pub struct Kilometers(i32);

impl Kilometers {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    // конвертация километров в метры
    pub fn to_meters(&self) -> Meters {
        // Умножьте значение на 1000, чтобы перевести в метры
        Meters(self.0 * 1000)
    }

    pub fn get_value(&self) -> i32 {
        self.0
    }
}

pub struct Meters(i32);

impl Meters {
    pub fn get_value(&self) -> i32 {
        self.0
    }
}
