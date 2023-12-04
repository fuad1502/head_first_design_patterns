// Light
#[derive(Copy, Clone, Debug)]
pub enum LightState {
    ON,
    OFF,
}

pub struct Light {
    name: String,
    state: LightState,
}

impl Light {
    pub fn new(name: String) -> Light {
        Light {
            name,
            state: LightState::OFF,
        }
    }
    pub fn on(&mut self) {
        println!("Turned {} light on", self.name);
        self.state = LightState::ON;
    }
    pub fn off(&mut self) {
        println!("Turned {} light off", self.name);
        self.state = LightState::OFF;
    }
    pub fn get_state(&self) -> LightState {
        self.state
    }
}

// CeilingFan
#[derive(Copy, Clone, Debug)]
pub enum FanSpeed {
    HIGH,
    MEDIUM,
    LOW,
    OFF,
}

pub struct CeilingFan {
    name: String,
    speed: FanSpeed,
}

impl CeilingFan {
    pub fn new(name: String) -> CeilingFan {
        CeilingFan {
            name,
            speed: FanSpeed::OFF,
        }
    }
    pub fn high(&mut self) {
        println!("Setting {} fan to high", self.name);
        self.speed = FanSpeed::HIGH;
    }
    pub fn medium(&mut self) {
        println!("Setting {} fan to medium", self.name);
        self.speed = FanSpeed::MEDIUM;
    }
    pub fn low(&mut self) {
        println!("Setting {} fan to low", self.name);
        self.speed = FanSpeed::LOW;
    }
    pub fn off(&mut self) {
        println!("Setting {} fan to off", self.name);
        self.speed = FanSpeed::OFF;
    }
    pub fn get_speed(&self) -> FanSpeed {
        self.speed
    }
}
