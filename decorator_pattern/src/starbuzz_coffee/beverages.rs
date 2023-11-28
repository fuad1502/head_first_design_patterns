use super::{Beverage, BeverageSize};

// Espresso
pub struct Espresso {
    size: BeverageSize,
}

impl Espresso {
    pub fn new(size: BeverageSize) -> Espresso {
        Espresso { size }
    }
}

impl Beverage for Espresso {
    fn description(&self) -> String {
        "Espresso".to_string()
    }

    fn cost(&self) -> f64 {
        1.99
    }

    fn get_size(&self) -> BeverageSize {
        self.size
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.size = size;
    }
}

// HouseBlend
pub struct HouseBlend {
    size: BeverageSize,
}

impl HouseBlend {
    pub fn new(size: BeverageSize) -> HouseBlend {
        HouseBlend { size }
    }
}

impl Beverage for HouseBlend {
    fn description(&self) -> String {
        "House Blend".to_string()
    }

    fn cost(&self) -> f64 {
        0.89
    }

    fn get_size(&self) -> BeverageSize {
        self.size
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.size = size;
    }
}

// DarkRoast
pub struct DarkRoast {
    size: BeverageSize,
}

impl DarkRoast {
    pub fn new(size: BeverageSize) -> DarkRoast {
        DarkRoast { size }
    }
}

impl Beverage for DarkRoast {
    fn description(&self) -> String {
        "Dark Roast".to_string()
    }

    fn cost(&self) -> f64 {
        0.99
    }

    fn get_size(&self) -> BeverageSize {
        self.size
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.size = size;
    }
}
