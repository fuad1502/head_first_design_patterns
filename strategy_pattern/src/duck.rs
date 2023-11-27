pub mod fly_behaviours;
pub mod quack_behaviours;

use fly_behaviours::*;
use quack_behaviours::*;

// Define QuackBehaviour trait
pub trait QuackBehaviour {
    fn quack(&self);
}

// Define FlyBehaviour trait
pub trait FlyBehaviour {
    fn fly(&self);
}

// Duck trait
pub trait Duck {
    fn quack(&self);
    fn fly(&self);
    fn display(&self);
    fn set_quack_behaviour(&mut self, quack_behaviour: Box<dyn QuackBehaviour>);
    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>);
}

// Implement various Ducks
// TODO: Create derive macro
pub struct MallardDuck {
    quack_behaviour: Box<dyn QuackBehaviour>,
    fly_behaviour: Box<dyn FlyBehaviour>,
}
impl MallardDuck {
    pub fn new() -> MallardDuck {
        let quack_behaviour = Box::new(Quack {});
        let fly_behaviour = Box::new(FlyWithWings {});
        MallardDuck {
            quack_behaviour,
            fly_behaviour,
        }
    }
}
impl Duck for MallardDuck {
    fn display(&self) {
        println!("I am a MallardDuck! 🦆");
    }
    fn quack(&self) {
        self.quack_behaviour.quack();
    }
    fn fly(&self) {
        self.fly_behaviour.fly();
    }
    fn set_quack_behaviour(&mut self, quack_behaviour: Box<dyn QuackBehaviour>) {
        self.quack_behaviour = quack_behaviour;
    }
    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour;
    }
}
pub struct RubberDuck {
    quack_behaviour: Box<dyn QuackBehaviour>,
    fly_behaviour: Box<dyn FlyBehaviour>,
}
impl RubberDuck {
    pub fn new() -> RubberDuck {
        let quack_behaviour = Box::new(Squeak {});
        let fly_behaviour = Box::new(FlyNoWay {});
        RubberDuck {
            quack_behaviour,
            fly_behaviour,
        }
    }
}
impl Duck for RubberDuck {
    fn display(&self) {
        println!("I am a RubberDuck! 🦆");
    }
    fn quack(&self) {
        self.quack_behaviour.quack();
    }
    fn fly(&self) {
        self.fly_behaviour.fly();
    }
    fn set_quack_behaviour(&mut self, quack_behaviour: Box<dyn QuackBehaviour>) {
        self.quack_behaviour = quack_behaviour;
    }
    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour;
    }
}
