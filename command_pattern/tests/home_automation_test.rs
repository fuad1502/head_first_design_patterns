use std::cell::RefCell;
use std::rc::Rc;

use command_pattern::home_automation::{command::*, receiver::*, *};

#[test]
fn home_automation_test() {
    println!("======[ home_automation_test.rs start ]======");
    let bedroom_light = Rc::new(RefCell::new(Light::new("Bed Room".to_string())));
    let kitched_light = Rc::new(RefCell::new(Light::new("Kitched".to_string())));
    let livingroom_fan = Rc::new(RefCell::new(CeilingFan::new("Living Room".to_string())));
    let mut remote = Remote::new();
    remote.set_command(
        0,
        Box::new(LightOnCommand::new(Rc::clone(&bedroom_light))),
        Box::new(LightOffCommand::new(Rc::clone(&bedroom_light))),
    );
    remote.set_command(
        1,
        Box::new(LightOnCommand::new(Rc::clone(&kitched_light))),
        Box::new(LightOffCommand::new(Rc::clone(&kitched_light))),
    );
    remote.set_command(
        2,
        Box::new(FanHighCommand::new(Rc::clone(&livingroom_fan))),
        Box::new(FanOffCommand::new(Rc::clone(&livingroom_fan))),
    );
    remote.set_command(
        3,
        Box::new(FanMediumCommand::new(Rc::clone(&livingroom_fan))),
        Box::new(FanOffCommand::new(Rc::clone(&livingroom_fan))),
    );
    remote.press_on_button(0);
    remote.press_on_button(1);
    remote.press_off_button(0);
    remote.press_undo_button();
    remote.press_undo_button();
    remote.press_on_button(3);
    remote.press_on_button(2);
    remote.press_off_button(2);
    remote.press_undo_button();
    remote.press_undo_button();
    remote.press_undo_button();
    remote.press_undo_button();
    remote.press_undo_button();
    println!("======[ home_automation_test.rs end ]======");
}
