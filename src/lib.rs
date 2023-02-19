pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub struct Tuple(u32, f32, f64);

impl Tuple {
    pub fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    pub fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    pub fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }

    pub fn is_default(&self) -> bool {
        self.0 == 0 && self.1 == 0.0 && self.2 == 0.0
    }

    pub fn sum(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

pub struct Array([f64; 3]);

impl Array {
    pub fn default_values() -> Self {
        Self([0.0; 3])
    }

    pub fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    pub fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }

    pub fn is_default(&self) -> bool {
        for value in &self.0 {
            if *value != 0.0 {
                return false;
            }
        }
        true
    }

    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for value in &self.0 {
            sum += *value;
        }
        sum
    }
}
