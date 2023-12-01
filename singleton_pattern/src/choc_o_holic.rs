use std::sync::{Mutex, OnceLock};

pub struct ChocolateBoiler {
    empty: bool,
    boiled: bool,
}

impl ChocolateBoiler {
    fn new() -> ChocolateBoiler {
        ChocolateBoiler {
            empty: true,
            boiled: false,
        }
    }
    pub fn get_boiler() -> &'static Mutex<ChocolateBoiler> {
        static BOILER: OnceLock<Mutex<ChocolateBoiler>> = OnceLock::new();
        BOILER.get_or_init(|| Mutex::new(ChocolateBoiler::new()))
    }
    pub fn fill(&mut self) {
        if self.empty {
            self.empty = false;
            println!("Filling boiler...");
        } else {
            println!("Boiler is full!");
        }
    }
    pub fn boil(&mut self) {
        if !self.empty && !self.boiled {
            self.boiled = true;
            println!("Boiling boiler...");
        } else if !self.empty {
            println!("Boiler is already boiling!");
        } else {
            println!("Boiler is empty!");
        }
    }
    pub fn drain(&mut self) {
        if !self.empty && self.boiled {
            self.empty = true;
            println!("Draining boiler...");
        } else if !self.empty {
            println!("Boiler is not yet boiled!");
        } else {
            println!("Boiler is empty!");
        }
    }
    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn is_boiled(&self) -> bool {
        self.boiled
    }
}
