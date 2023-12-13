use std::cell::RefCell;

use rand::prelude::*;

pub trait State {
    fn insert_quarter(&self) -> Box<dyn State>;
    fn eject_quarter(&self) -> Box<dyn State>;
    fn turn_crank(&self) -> Box<dyn State>;
    fn dispense_ball(&self) -> Box<dyn State>;
    fn refill(&self, gumballs: usize) -> Box<dyn State>;
}

pub struct GumballMachine {
    state: Option<Box<dyn State>>,
}

impl GumballMachine {
    pub fn new(gumballs: usize) -> GumballMachine {
        if gumballs > 0 {
            GumballMachine {
                state: Some(Box::new(NoQuarter::new(gumballs))),
            }
        } else {
            GumballMachine {
                state: Some(Box::new(OutofGumballs::new(gumballs))),
            }
        }
    }

    pub fn insert_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_quarter());
        }
    }

    pub fn eject_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.eject_quarter());
        }
    }

    pub fn turn_crank(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.turn_crank());
        }
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense_ball());
        }
    }

    pub fn refill(&mut self, gumballs: usize) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.refill(gumballs));
        }
    }
}

struct NoQuarter {
    gumballs: usize,
}

impl NoQuarter {
    fn new(gumballs: usize) -> NoQuarter {
        NoQuarter { gumballs }
    }
}

impl State for NoQuarter {
    fn insert_quarter(&self) -> Box<dyn State> {
        println!("Inserted quarter...");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn refill(&self, gumballs: usize) -> Box<dyn State> {
        println!("Refilled machine with {} gumballs...", gumballs);
        let gumballs = self.gumballs + gumballs;
        Box::new(NoQuarter::new(gumballs))
    }
}

struct HasQuarter {
    gumballs: usize,
    rng: RefCell<ThreadRng>,
}

impl HasQuarter {
    fn new(gumballs: usize) -> HasQuarter {
        let rng = RefCell::new(rand::thread_rng());
        HasQuarter { gumballs, rng }
    }
}

impl State for HasQuarter {
    fn insert_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State> {
        println!("Ejected quarter...");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State> {
        println!("Turned crank...");
        let win = self.rng.borrow_mut().gen_bool(1.0 / 10.0);
        if win && self.gumballs >= 2 {
            println!("We got a winner! 🎊");
            Box::new(Winner::new(self.gumballs))
        } else {
            Box::new(GumballSold::new(self.gumballs))
        }
    }

    fn dispense_ball(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }
}

struct GumballSold {
    gumballs: usize,
}

impl GumballSold {
    fn new(gumballs: usize) -> GumballSold {
        GumballSold { gumballs }
    }
}

impl State for GumballSold {
    fn insert_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State> {
        println!("Dispensed 1 ball...");
        let gumballs = self.gumballs - 1;
        if gumballs == 0 {
            Box::new(OutofGumballs::new(gumballs))
        } else {
            Box::new(NoQuarter::new(gumballs))
        }
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }
}

struct Winner {
    gumballs: usize,
}

impl Winner {
    fn new(gumballs: usize) -> Winner {
        Winner { gumballs }
    }
}

impl State for Winner {
    fn insert_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State> {
        println!("Dispensed 2 balls...");
        let gumballs = self.gumballs - 2;
        if gumballs == 0 {
            Box::new(OutofGumballs::new(gumballs))
        } else {
            Box::new(NoQuarter::new(gumballs))
        }
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }
}

struct OutofGumballs {
    gumballs: usize,
}

impl OutofGumballs {
    fn new(gumballs: usize) -> OutofGumballs {
        OutofGumballs { gumballs }
    }
}

impl State for OutofGumballs {
    fn insert_quarter(&self) -> Box<dyn State> {
        println!("Sorry, machine is out of gumballs...");
        Box::new(OutofGumballs::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(OutofGumballs::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(OutofGumballs::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State> {
        println!("Error!");
        Box::new(OutofGumballs::new(self.gumballs))
    }

    fn refill(&self, gumballs: usize) -> Box<dyn State> {
        println!("Refilled machine with {} gumballs...", gumballs);
        let gumballs = self.gumballs + gumballs;
        if gumballs > 0 {
            Box::new(NoQuarter::new(gumballs))
        } else {
            Box::new(OutofGumballs::new(gumballs))
        }
    }
}
