use super::*;
use std::cell::RefCell;

// NoCommand
pub struct NoCommand {}

impl NoCommand {
    pub fn new() -> NoCommand {
        NoCommand {}
    }
}

impl Command for NoCommand {
    fn execute(&self) {}

    fn undo(&self) {}
}

// LightOnCommand
pub struct LightOnCommand {
    light: Rc<RefCell<receiver::Light>>,
    previous_states: RefCell<Vec<receiver::LightState>>,
}

impl LightOnCommand {
    pub fn new(light: Rc<RefCell<receiver::Light>>) -> LightOnCommand {
        LightOnCommand {
            light,
            previous_states: RefCell::new(vec![]),
        }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        let previous_state = self.light.borrow().get_state();
        self.previous_states.borrow_mut().push(previous_state);
        self.light.borrow_mut().on();
    }

    fn undo(&self) {
        if let Some(previous_state) = self.previous_states.borrow_mut().pop() {
            match previous_state {
                receiver::LightState::ON => self.light.borrow_mut().on(),
                receiver::LightState::OFF => self.light.borrow_mut().off(),
            }
        }
    }
}

// LightOffCommand
pub struct LightOffCommand {
    light: Rc<RefCell<receiver::Light>>,
    previous_states: RefCell<Vec<receiver::LightState>>,
}

impl LightOffCommand {
    pub fn new(light: Rc<RefCell<receiver::Light>>) -> LightOffCommand {
        LightOffCommand {
            light,
            previous_states: RefCell::new(vec![]),
        }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        let previous_state = self.light.borrow().get_state();
        self.previous_states.borrow_mut().push(previous_state);
        self.light.borrow_mut().off();
    }

    fn undo(&self) {
        if let Some(previous_state) = self.previous_states.borrow_mut().pop() {
            match previous_state {
                receiver::LightState::ON => self.light.borrow_mut().on(),
                receiver::LightState::OFF => self.light.borrow_mut().off(),
            }
        }
    }
}

// FanHighCommand
pub struct FanHighCommand {
    fan: Rc<RefCell<receiver::CeilingFan>>,
    previous_speeds: RefCell<Vec<receiver::FanSpeed>>,
}

impl FanHighCommand {
    pub fn new(fan: Rc<RefCell<receiver::CeilingFan>>) -> FanHighCommand {
        FanHighCommand {
            fan,
            previous_speeds: RefCell::new(vec![]),
        }
    }
}

impl Command for FanHighCommand {
    fn execute(&self) {
        let previous_speed = self.fan.borrow().get_speed();
        self.previous_speeds.borrow_mut().push(previous_speed);
        self.fan.borrow_mut().high();
    }

    fn undo(&self) {
        if let Some(previous_speed) = self.previous_speeds.borrow_mut().pop() {
            match previous_speed {
                receiver::FanSpeed::HIGH => self.fan.borrow_mut().high(),
                receiver::FanSpeed::MEDIUM => self.fan.borrow_mut().medium(),
                receiver::FanSpeed::LOW => self.fan.borrow_mut().low(),
                receiver::FanSpeed::OFF => self.fan.borrow_mut().off(),
            }
        }
    }
}

// FanMediumCommand
pub struct FanMediumCommand {
    fan: Rc<RefCell<receiver::CeilingFan>>,
    previous_speeds: RefCell<Vec<receiver::FanSpeed>>,
}

impl FanMediumCommand {
    pub fn new(fan: Rc<RefCell<receiver::CeilingFan>>) -> FanMediumCommand {
        FanMediumCommand {
            fan,
            previous_speeds: RefCell::new(vec![]),
        }
    }
}

impl Command for FanMediumCommand {
    fn execute(&self) {
        let previous_speed = self.fan.borrow().get_speed();
        self.previous_speeds.borrow_mut().push(previous_speed);
        self.fan.borrow_mut().medium();
    }

    fn undo(&self) {
        if let Some(previous_speed) = self.previous_speeds.borrow_mut().pop() {
            match previous_speed {
                receiver::FanSpeed::HIGH => self.fan.borrow_mut().high(),
                receiver::FanSpeed::MEDIUM => self.fan.borrow_mut().medium(),
                receiver::FanSpeed::LOW => self.fan.borrow_mut().low(),
                receiver::FanSpeed::OFF => self.fan.borrow_mut().off(),
            }
        }
    }
}

// FanLowCommand
pub struct FanLowCommand {
    fan: Rc<RefCell<receiver::CeilingFan>>,
    previous_speeds: RefCell<Vec<receiver::FanSpeed>>,
}

impl FanLowCommand {
    pub fn new(fan: Rc<RefCell<receiver::CeilingFan>>) -> FanLowCommand {
        FanLowCommand {
            fan,
            previous_speeds: RefCell::new(vec![]),
        }
    }
}

impl Command for FanLowCommand {
    fn execute(&self) {
        let previous_speed = self.fan.borrow().get_speed();
        self.previous_speeds.borrow_mut().push(previous_speed);
        self.fan.borrow_mut().low();
    }

    fn undo(&self) {
        if let Some(previous_speed) = self.previous_speeds.borrow_mut().pop() {
            match previous_speed {
                receiver::FanSpeed::HIGH => self.fan.borrow_mut().high(),
                receiver::FanSpeed::MEDIUM => self.fan.borrow_mut().medium(),
                receiver::FanSpeed::LOW => self.fan.borrow_mut().low(),
                receiver::FanSpeed::OFF => self.fan.borrow_mut().off(),
            }
        }
    }
}

// FanOffCommand
pub struct FanOffCommand {
    fan: Rc<RefCell<receiver::CeilingFan>>,
    previous_speeds: RefCell<Vec<receiver::FanSpeed>>,
}

impl FanOffCommand {
    pub fn new(fan: Rc<RefCell<receiver::CeilingFan>>) -> FanOffCommand {
        FanOffCommand {
            fan,
            previous_speeds: RefCell::new(vec![]),
        }
    }
}

impl Command for FanOffCommand {
    fn execute(&self) {
        let previous_speed = self.fan.borrow().get_speed();
        self.previous_speeds.borrow_mut().push(previous_speed);
        self.fan.borrow_mut().off();
    }

    fn undo(&self) {
        if let Some(previous_speed) = self.previous_speeds.borrow_mut().pop() {
            match previous_speed {
                receiver::FanSpeed::HIGH => self.fan.borrow_mut().high(),
                receiver::FanSpeed::MEDIUM => self.fan.borrow_mut().medium(),
                receiver::FanSpeed::LOW => self.fan.borrow_mut().low(),
                receiver::FanSpeed::OFF => self.fan.borrow_mut().off(),
            }
        }
    }
}
