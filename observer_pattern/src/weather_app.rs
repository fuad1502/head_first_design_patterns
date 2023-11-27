use std::cell::RefCell;
use std::rc::Rc;

pub trait Subject {
    fn add_observer(&mut self, observer: Rc<RefCell<Box<dyn Observer>>>);
    fn notify_observers(&self);
}

// TODO: Let observer grab the required information from the subject themselves
pub trait Observer {
    fn update(&mut self, temperature: f64, humidity: f64);
}

pub trait Display {
    fn display(&self);
}

pub struct WeatherData {
    observers: Vec<Rc<RefCell<Box<dyn Observer>>>>,
    temperature: f64,
    humidity: f64,
}

impl WeatherData {
    pub fn new() -> WeatherData {
        WeatherData {
            observers: vec![],
            temperature: 0.0,
            humidity: 0.0,
        }
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }

    pub fn get_humidity(&self) -> f64 {
        self.humidity
    }

    pub fn set_data(&mut self, temperature: f64, humidity: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.notify_observers();
    }
}

impl Subject for WeatherData {
    fn add_observer(&mut self, observer: Rc<RefCell<Box<dyn Observer>>>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        for o in self.observers.iter() {
            o.borrow_mut().update(self.temperature, self.humidity);
        }
    }
}

pub struct CurrentConditionsDisplay {
    temperature: f64,
    humidity: f64,
}

impl CurrentConditionsDisplay {
    pub fn new() -> CurrentConditionsDisplay {
        CurrentConditionsDisplay {
            temperature: 0.0,
            humidity: 0.0,
        }
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f64, humidity: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.display();
    }
}

impl Display for CurrentConditionsDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {}°C and {}% humidity",
            self.temperature, self.humidity
        );
    }
}

pub struct AverageConditionsDisplay {
    update_count: i64,
    average_temperature: f64,
    max_temperature: f64,
    min_temperature: f64,
}

impl AverageConditionsDisplay {
    pub fn new() -> AverageConditionsDisplay {
        AverageConditionsDisplay {
            update_count: 0,
            average_temperature: 0.0,
            max_temperature: f64::MIN,
            min_temperature: f64::MAX,
        }
    }
}

impl Observer for AverageConditionsDisplay {
    fn update(&mut self, temperature: f64, _humidity: f64) {
        if temperature > self.max_temperature {
            self.max_temperature = temperature;
        }
        if temperature < self.min_temperature {
            self.min_temperature = temperature;
        }
        let current_count = self.update_count as f64;
        self.average_temperature =
            (self.average_temperature * current_count + temperature) / (current_count + 1.0);
        self.update_count += 1;
        self.display();
    }
}

impl Display for AverageConditionsDisplay {
    fn display(&self) {
        println!(
            "Avg/Min/Max temperature: {}/{}/{} °C",
            self.average_temperature, self.min_temperature, self.max_temperature
        );
    }
}
