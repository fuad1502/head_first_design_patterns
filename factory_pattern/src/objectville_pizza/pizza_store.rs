use super::ingredient_factory::*;
use super::pizza::*;
use super::*;
use pizza_store_macro::PizzaStore;
use std::rc::Rc;

// Pizza Store implementations
// New York Pizza Store
#[derive(PizzaStore)]
pub struct NYPizzaStore {
    ingredient_factory: Rc<Box<dyn IngredientFactory>>,
}
impl NYPizzaStore {
    pub fn new() -> NYPizzaStore {
        NYPizzaStore {
            ingredient_factory: Rc::new(Box::new(NYIngredientFactory {})),
        }
    }
}
impl PizzaCreator for NYPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
        match pizza_type {
            "cheese" => Some(Box::new(CheesePizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            "clam" => Some(Box::new(ClamPizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            _ => None,
        }
    }
}
// Chicago Pizza Store
#[derive(PizzaStore)]
pub struct ChicagoPizzaStore {
    ingredient_factory: Rc<Box<dyn IngredientFactory>>,
}
impl ChicagoPizzaStore {
    pub fn new() -> ChicagoPizzaStore {
        ChicagoPizzaStore {
            ingredient_factory: Rc::new(Box::new(ChicagoIngredientFactory {})),
        }
    }
}
impl PizzaCreator for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
        match pizza_type {
            "cheese" => Some(Box::new(CheesePizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            "clam" => Some(Box::new(ClamPizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            _ => None,
        }
    }
}
// California Pizza Store
#[derive(PizzaStore)]
pub struct CaliforniaPizzaStore {
    ingredient_factory: Rc<Box<dyn IngredientFactory>>,
}
impl CaliforniaPizzaStore {
    pub fn new() -> CaliforniaPizzaStore {
        CaliforniaPizzaStore {
            ingredient_factory: Rc::new(Box::new(CaliforniaIngredientFactory {})),
        }
    }
}
impl PizzaCreator for CaliforniaPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
        match pizza_type {
            "cheese" => Some(Box::new(CheesePizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            "clam" => Some(Box::new(ClamPizza::new(Rc::clone(
                &self.ingredient_factory,
            )))),
            _ => None,
        }
    }
}
