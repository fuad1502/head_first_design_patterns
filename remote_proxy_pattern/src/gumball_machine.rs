use rand::prelude::*;

pub trait State {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send>;
    fn eject_quarter(&self) -> Box<dyn State + Sync + Send>;
    fn turn_crank(&self) -> Box<dyn State + Sync + Send>;
    fn dispense_ball(&self) -> Box<dyn State + Sync + Send>;
    fn refill(&self, gumballs: usize) -> Box<dyn State + Sync + Send>;
    fn get_state(&self) -> GumballState;
    fn get_count(&self) -> usize;
}

pub trait GumballMachineInterface {
    fn insert_quarter(&mut self);
    fn eject_quarter(&mut self);
    fn turn_crank(&mut self);
    fn refill(&mut self, gumballs: usize);
    fn get_state(&self) -> Option<GumballState>;
    fn get_count(&self) -> Option<usize>;
    fn get_location(&self) -> String;
}

#[derive(Copy, Clone, Debug)]
pub enum GumballState {
    NoQuarter,
    HasQuarter,
    GumballSold,
    Winner,
    OutOfGumballs,
}

pub struct GumballMachine {
    state: Option<Box<dyn State + Sync + Send>>,
    location: String,
}

impl GumballMachine {
    pub fn new(gumballs: usize, location: String) -> GumballMachine {
        if gumballs > 0 {
            GumballMachine {
                state: Some(Box::new(NoQuarter::new(gumballs))),
                location,
            }
        } else {
            GumballMachine {
                state: Some(Box::new(OutOfGumballs::new(gumballs))),
                location,
            }
        }
    }
}

impl GumballMachineInterface for GumballMachine {
    fn insert_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_quarter());
        }
    }

    fn eject_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.eject_quarter());
        }
    }

    fn turn_crank(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.turn_crank());
        }
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense_ball());
        }
    }

    fn refill(&mut self, gumballs: usize) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.refill(gumballs));
        }
    }

    fn get_state(&self) -> Option<GumballState> {
        if let Some(state) = &self.state {
            Some(state.get_state())
        } else {
            None
        }
    }

    fn get_count(&self) -> Option<usize> {
        if let Some(state) = &self.state {
            Some(state.get_count())
        } else {
            None
        }
    }

    fn get_location(&self) -> String {
        self.location.clone()
    }
}

struct NoQuarter {
    gumballs: usize,
    state: GumballState,
}

impl NoQuarter {
    fn new(gumballs: usize) -> NoQuarter {
        NoQuarter {
            gumballs,
            state: GumballState::NoQuarter,
        }
    }
}

impl State for NoQuarter {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Inserted quarter...");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn refill(&self, gumballs: usize) -> Box<dyn State + Sync + Send> {
        println!("Refilled machine with {} gumballs...", gumballs);
        let gumballs = self.gumballs + gumballs;
        Box::new(NoQuarter::new(gumballs))
    }

    fn get_state(&self) -> GumballState {
        self.state
    }

    fn get_count(&self) -> usize {
        self.gumballs
    }
}

struct HasQuarter {
    gumballs: usize,
    state: GumballState,
}

impl HasQuarter {
    fn new(gumballs: usize) -> HasQuarter {
        HasQuarter {
            gumballs,
            state: GumballState::HasQuarter,
        }
    }
}

impl State for HasQuarter {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Ejected quarter...");
        Box::new(NoQuarter::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State + Sync + Send> {
        println!("Turned crank...");
        let mut rng = rand::thread_rng();
        let win = rng.gen_bool(1.0 / 10.0);
        if win && self.gumballs >= 2 {
            println!("We got a winner! 🎊");
            Box::new(Winner::new(self.gumballs))
        } else {
            Box::new(GumballSold::new(self.gumballs))
        }
    }

    fn dispense_ball(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(HasQuarter::new(self.gumballs))
    }

    fn get_state(&self) -> GumballState {
        self.state
    }

    fn get_count(&self) -> usize {
        self.gumballs
    }
}

struct GumballSold {
    gumballs: usize,
    state: GumballState,
}

impl GumballSold {
    fn new(gumballs: usize) -> GumballSold {
        GumballSold {
            gumballs,
            state: GumballState::GumballSold,
        }
    }
}

impl State for GumballSold {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State + Sync + Send> {
        println!("Dispensed 1 ball...");
        let gumballs = self.gumballs - 1;
        if gumballs == 0 {
            Box::new(OutOfGumballs::new(gumballs))
        } else {
            Box::new(NoQuarter::new(gumballs))
        }
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(GumballSold::new(self.gumballs))
    }

    fn get_state(&self) -> GumballState {
        self.state
    }

    fn get_count(&self) -> usize {
        self.gumballs
    }
}

struct Winner {
    gumballs: usize,
    state: GumballState,
}

impl Winner {
    fn new(gumballs: usize) -> Winner {
        Winner {
            gumballs,
            state: GumballState::Winner,
        }
    }
}

impl State for Winner {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State + Sync + Send> {
        println!("Dispensed 2 balls...");
        let gumballs = self.gumballs - 2;
        if gumballs == 0 {
            Box::new(OutOfGumballs::new(gumballs))
        } else {
            Box::new(NoQuarter::new(gumballs))
        }
    }

    fn refill(&self, _gumballs: usize) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(Winner::new(self.gumballs))
    }

    fn get_state(&self) -> GumballState {
        self.state
    }

    fn get_count(&self) -> usize {
        self.gumballs
    }
}

struct OutOfGumballs {
    gumballs: usize,
    state: GumballState,
}

impl OutOfGumballs {
    fn new(gumballs: usize) -> OutOfGumballs {
        OutOfGumballs {
            gumballs,
            state: GumballState::OutOfGumballs,
        }
    }
}

impl State for OutOfGumballs {
    fn insert_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Sorry, machine is out of gumballs...");
        Box::new(OutOfGumballs::new(self.gumballs))
    }

    fn eject_quarter(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(OutOfGumballs::new(self.gumballs))
    }

    fn turn_crank(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(OutOfGumballs::new(self.gumballs))
    }

    fn dispense_ball(&self) -> Box<dyn State + Sync + Send> {
        println!("Error!");
        Box::new(OutOfGumballs::new(self.gumballs))
    }

    fn refill(&self, gumballs: usize) -> Box<dyn State + Sync + Send> {
        println!("Refilled machine with {} gumballs...", gumballs);
        let gumballs = self.gumballs + gumballs;
        if gumballs > 0 {
            Box::new(NoQuarter::new(gumballs))
        } else {
            Box::new(OutOfGumballs::new(gumballs))
        }
    }

    fn get_state(&self) -> GumballState {
        self.state
    }

    fn get_count(&self) -> usize {
        self.gumballs
    }
}
