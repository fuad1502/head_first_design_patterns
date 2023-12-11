use std::io;

pub trait CaffeinBeverage {
    fn prepare_recipe(&self) {
        self.boil_water();
        self.brew();
        self.pour();
        if self.customer_wants_condiments() {
            self.add_condiment();
        }
    }
    fn boil_water(&self) {
        println!("Boiling water");
    }
    fn pour(&self) {
        println!("Pouring into cup");
    }

    // Method to implement
    fn brew(&self);
    fn add_condiment(&self);

    // Hook
    fn customer_wants_condiments(&self) -> bool {
        true
    }
}

pub struct Coffee {}
impl Coffee {
    pub fn new() -> Coffee {
        Coffee {}
    }
}

impl CaffeinBeverage for Coffee {
    fn brew(&self) {
        println!("Dripping coffee through filter");
    }

    fn add_condiment(&self) {
        println!("Adding sugar and milk");
    }

    // Override hook
    fn customer_wants_condiments(&self) -> bool {
        let mut answer = "".to_string();
        while answer != "y".to_string() && answer != "n".to_string() {
            answer = "".to_string();
            println!("Would you like milk and sugar with your coffee? (y/n)");
            io::stdin()
                .read_line(&mut answer)
                .expect("Cannot read from standard input");
            answer = answer.trim().to_string().to_lowercase();
        }
        answer == "y".to_string()
    }
}
