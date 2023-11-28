pub mod beverages;
pub mod condiments;

pub trait Beverage {
    fn get_size(&self) -> BeverageSize;
    fn set_size(&mut self, size: BeverageSize);
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

pub trait CondimentDecorator {
    fn decorate(beverage: Box<dyn Beverage>) -> Box<dyn Beverage>;
}

#[derive(Clone, Copy, Debug)]
pub enum BeverageSize {
    TALL,
    GRANDE,
    VENTI,
}
