use observer_pattern::weather_app::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn weather_app_test() {
    println!("======[ weather_app_test.rs start ]======");
    let mut weather_data = WeatherData::new();
    let current_conditions_display: Rc<RefCell<Box<dyn Observer>>> =
        Rc::new(RefCell::new(Box::new(CurrentConditionsDisplay::new())));
    let average_conditions_display: Rc<RefCell<Box<dyn Observer>>> =
        Rc::new(RefCell::new(Box::new(AverageConditionsDisplay::new())));
    weather_data.add_observer(Rc::clone(&current_conditions_display));
    weather_data.add_observer(Rc::clone(&average_conditions_display));
    weather_data.set_data(40.0, 70.0);
    weather_data.set_data(50.0, 80.0);
    weather_data.set_data(30.0, 90.0);
    println!("======[ weather_app_test.rs end ]======");
}
