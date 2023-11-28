use super::{Beverage, BeverageSize, CondimentDecorator};

// Mocha
pub struct Mocha {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Mocha {
    fn decorate(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        let mocha = Mocha { beverage };
        Box::new(mocha)
    }
}

impl Beverage for Mocha {
    fn description(&self) -> String {
        self.beverage.description() + ", Mocha"
    }

    fn get_size(&self) -> BeverageSize {
        self.beverage.get_size()
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.beverage.set_size(size);
    }

    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.2
    }
}

// Whip
pub struct Whip {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Whip {
    fn decorate(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        let mocha = Whip { beverage };
        Box::new(mocha)
    }
}

impl Beverage for Whip {
    fn description(&self) -> String {
        self.beverage.description() + ", Whip"
    }

    fn get_size(&self) -> BeverageSize {
        self.beverage.get_size()
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.beverage.set_size(size);
    }

    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.1
    }
}

// Soy
pub struct Soy {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Soy {
    fn decorate(beverage: Box<dyn Beverage>) -> Box<dyn Beverage> {
        let mocha = Soy { beverage };
        Box::new(mocha)
    }
}

impl Beverage for Soy {
    fn description(&self) -> String {
        self.beverage.description() + ", Soy"
    }

    fn get_size(&self) -> BeverageSize {
        self.beverage.get_size()
    }

    fn set_size(&mut self, size: BeverageSize) {
        self.beverage.set_size(size);
    }

    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.15
    }
}
