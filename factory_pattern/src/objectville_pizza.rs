mod ingredient_factory;
mod pizza;
pub mod pizza_store;

// Pizza trait
pub trait Pizza {
    fn prepare(&mut self);
    fn bake(&self) {
        println!("Bake for 25 minutes at 350");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagonal slices");
    }
    fn package(&self) {
        println!("Place pizza in official Objectville Pizza box");
    }
}
// Pizza Store trait
pub trait PizzaStore {
    fn order_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>>;
}
// Pizza Creator trait (FACTORY METHOD PATTERN)
trait PizzaCreator {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>>;
}
