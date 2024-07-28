#[derive(Copy, Clone)]
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

#[derive(Debug, PartialEq)]
pub struct Tuple(u32, f32, f64);

#[derive(Debug, PartialEq)]
pub struct Array([f64; 3]);

trait SomeTrait: Sized {
    fn sum(&self) -> f64 {
        let mut sum = 0_f64;
        let items = [Item::First, Item::Second, Item::Third];
        for item in items {
            sum += self.get_item(item) as f64
        }
        sum
    }

    fn is_default(&self) -> bool {
        let items = [Item::First, Item::Second, Item::Third];
        let default = Self::default_values();
        for item in items {
            if self.get_item(item) != default.get_item(item) {
                return false;
            }
        }
        true
    }

    fn default_values() -> Self;

    fn get_item(&self, item: Item) -> f64;

    fn set_item(&mut self, item: Item, value: f64);
}

impl SomeTrait for Array {
    fn default_values() -> Self {
        Self([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}

impl SomeTrait for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sum_abstract<T: SomeTrait>() {
        let mut rf = T::default_values();
        assert_eq!(rf.sum(), 0.);
        rf.set_item(Item::First, 1.);
        rf.set_item(Item::Second, 2.);
        rf.set_item(Item::Third, 3.);
        assert_eq!(rf.sum(), 6.);
    }

    #[test]
    fn test_sums() {
        test_sum_abstract::<Array>();
        test_sum_abstract::<Tuple>();
    }

    fn test_is_default_abstract<T: SomeTrait>() {
        let mut rf = T::default_values();
        assert_eq!(rf.is_default(), true);
        rf.set_item(Item::Second, 100.);
        assert_eq!(rf.is_default(), false);
    }

    #[test]
    fn test_is_defaults() {
        test_is_default_abstract::<Array>();
        test_is_default_abstract::<Tuple>();
    }

    #[test]
    fn test_default_values() {
        assert_eq!(Array::default_values(), Array([0., 0., 0.]));
        assert_eq!(Tuple::default_values(), Tuple(0, 0.0, 0.0))
    }

    fn test_set_get_item_abstract<T: SomeTrait>() {
        let mut rf = T::default_values();
        rf.set_item(Item::First, 64.);
        assert_eq!(rf.get_item(Item::First), 64.);

        rf.set_item(Item::Second, 12.);
        assert_eq!(rf.get_item(Item::Second), 12.);

        rf.set_item(Item::Third, 5.);
        assert_eq!(rf.get_item(Item::Third), 5.)
    }

    // tests for methods get_item / set_item merged to one due to theirs nature
    #[test]
    fn test_set_get_item() {
        test_set_get_item_abstract::<Array>();
        test_set_get_item_abstract::<Tuple>();
    }
}
