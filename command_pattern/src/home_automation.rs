pub mod command;
pub mod receiver;

use std::rc::Rc;

pub trait Command {
    fn execute(&self);
    fn undo(&self);
}

pub struct Remote {
    on_commands: Vec<Rc<Box<dyn Command>>>,
    off_commands: Vec<Rc<Box<dyn Command>>>,
    command_stack: Vec<Rc<Box<dyn Command>>>,
}

impl Remote {
    const NUM_OF_BUTTONS: usize = 7;

    pub fn new() -> Remote {
        let mut on_commands: Vec<Rc<Box<dyn Command>>> = vec![];
        let mut off_commands: Vec<Rc<Box<dyn Command>>> = vec![];
        let command_stack = vec![];
        for _ in 0..Self::NUM_OF_BUTTONS {
            on_commands.push(Rc::new(Box::new(command::NoCommand::new())));
            off_commands.push(Rc::new(Box::new(command::NoCommand::new())));
        }
        Remote {
            on_commands,
            off_commands,
            command_stack,
        }
    }

    pub fn set_command(
        &mut self,
        button_idx: usize,
        on_command: Box<dyn Command>,
        off_command: Box<dyn Command>,
    ) {
        if button_idx < Self::NUM_OF_BUTTONS {
            self.on_commands[button_idx] = Rc::new(on_command);
            self.off_commands[button_idx] = Rc::new(off_command);
        }
    }

    pub fn press_on_button(&mut self, button_idx: usize) {
        if button_idx < Self::NUM_OF_BUTTONS {
            println!("> On button {} pressed", button_idx);
            self.on_commands[button_idx].execute();
            self.command_stack
                .push(Rc::clone(&self.on_commands[button_idx]));
        }
    }

    pub fn press_off_button(&mut self, button_idx: usize) {
        if button_idx < Self::NUM_OF_BUTTONS {
            println!("> Off button {} pressed", button_idx);
            self.off_commands[button_idx].execute();
            self.command_stack
                .push(Rc::clone(&self.off_commands[button_idx]));
        }
    }

    pub fn press_undo_button(&mut self) {
        println!("> Undo button pressed");
        if let Some(command) = self.command_stack.pop() {
            command.undo();
        }
    }
}
