use super::ingredient_factory::*;
use super::*;
use std::rc::Rc;

// Pizza implementations
// Cheese Pizza
pub struct CheesePizza {
    ingredient_factory: Rc<Box<dyn IngredientFactory>>,
    ingredients: Vec<Box<dyn Ingredient>>,
}
impl CheesePizza {
    pub fn new(ingredient_factory: Rc<Box<dyn IngredientFactory>>) -> CheesePizza {
        CheesePizza {
            ingredient_factory,
            ingredients: vec![],
        }
    }
}
impl Pizza for CheesePizza {
    fn prepare(&mut self) {
        println!("Preparing...");
        self.ingredients
            .push(self.ingredient_factory.create_dough());
        self.ingredients
            .push(self.ingredient_factory.create_sauce());
        self.ingredients
            .push(self.ingredient_factory.create_cheese());
        for ingredient in self.ingredients.iter() {
            println!("> {}", ingredient.name());
        }
    }
}
// Clam Pizza
pub struct ClamPizza {
    ingredient_factory: Rc<Box<dyn IngredientFactory>>,
    ingredients: Vec<Box<dyn Ingredient>>,
}
impl ClamPizza {
    pub fn new(ingredient_factory: Rc<Box<dyn IngredientFactory>>) -> ClamPizza {
        ClamPizza {
            ingredient_factory,
            ingredients: vec![],
        }
    }
}
impl Pizza for ClamPizza {
    fn prepare(&mut self) {
        println!("Preparing...");
        self.ingredients
            .push(self.ingredient_factory.create_dough());
        self.ingredients
            .push(self.ingredient_factory.create_sauce());
        self.ingredients
            .push(self.ingredient_factory.create_cheese());
        self.ingredients
            .push(self.ingredient_factory.create_clams());
        for ingredient in self.ingredients.iter() {
            println!("> {}", ingredient.name());
        }
    }
}
