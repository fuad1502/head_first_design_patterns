// Ingredient trait
pub trait Ingredient {
    fn name(&self) -> String;
}
// Ingredient Factory trait (ABSTRACT FACTORY PATTERN)
pub trait IngredientFactory {
    fn create_dough(&self) -> Box<dyn Ingredient>;
    fn create_sauce(&self) -> Box<dyn Ingredient>;
    fn create_cheese(&self) -> Box<dyn Ingredient>;
    fn create_clams(&self) -> Box<dyn Ingredient>;
}

// Ingredient implementations
// Doughs
pub struct ThinCrustDough {}
impl Ingredient for ThinCrustDough {
    fn name(&self) -> String {
        "thin crust dough".to_string()
    }
}
pub struct ThickCrustDough {}
impl Ingredient for ThickCrustDough {
    fn name(&self) -> String {
        "thick crust dough".to_string()
    }
}
pub struct VeryThinCrustDough {}
impl Ingredient for VeryThinCrustDough {
    fn name(&self) -> String {
        "very thin crust dough".to_string()
    }
}
// Sauces
pub struct MarianaSauce {}
impl Ingredient for MarianaSauce {
    fn name(&self) -> String {
        "mariana sauce".to_string()
    }
}
pub struct PlumTomatoSauce {}
impl Ingredient for PlumTomatoSauce {
    fn name(&self) -> String {
        "plum tomato sauce".to_string()
    }
}
pub struct BruchettaSauce {}
impl Ingredient for BruchettaSauce {
    fn name(&self) -> String {
        "bruchetta sauce".to_string()
    }
}
// Cheese
pub struct ReggianoCheese {}
impl Ingredient for ReggianoCheese {
    fn name(&self) -> String {
        "reggiano cheese".to_string()
    }
}
pub struct GoatCheese {}
impl Ingredient for GoatCheese {
    fn name(&self) -> String {
        "goat cheese".to_string()
    }
}
pub struct MozarellaCheese {}
impl Ingredient for MozarellaCheese {
    fn name(&self) -> String {
        "mozarella cheese".to_string()
    }
}
// Clams
pub struct FreshClams {}
impl Ingredient for FreshClams {
    fn name(&self) -> String {
        "fresh clams".to_string()
    }
}
pub struct FrozenClams {}
impl Ingredient for FrozenClams {
    fn name(&self) -> String {
        "frozen clams".to_string()
    }
}

// Ingredient Factory implementations
pub struct NYIngredientFactory {}
impl IngredientFactory for NYIngredientFactory {
    fn create_dough(&self) -> Box<dyn Ingredient> {
        Box::new(ThinCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Ingredient> {
        Box::new(MarianaSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Ingredient> {
        Box::new(ReggianoCheese {})
    }
    fn create_clams(&self) -> Box<dyn Ingredient> {
        Box::new(FreshClams {})
    }
}
pub struct ChicagoIngredientFactory {}
impl IngredientFactory for ChicagoIngredientFactory {
    fn create_dough(&self) -> Box<dyn Ingredient> {
        Box::new(ThickCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Ingredient> {
        Box::new(PlumTomatoSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Ingredient> {
        Box::new(MozarellaCheese {})
    }
    fn create_clams(&self) -> Box<dyn Ingredient> {
        Box::new(FrozenClams {})
    }
}
pub struct CaliforniaIngredientFactory {}
impl IngredientFactory for CaliforniaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Ingredient> {
        Box::new(VeryThinCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Ingredient> {
        Box::new(BruchettaSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Ingredient> {
        Box::new(GoatCheese {})
    }
    fn create_clams(&self) -> Box<dyn Ingredient> {
        Box::new(FreshClams {})
    }
}
