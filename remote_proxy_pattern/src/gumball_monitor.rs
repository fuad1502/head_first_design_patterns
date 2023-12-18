use crate::GumballMachineInterface;

pub struct GumballMonitor {
    gumball_machine: Box<dyn GumballMachineInterface + Send>,
}

impl GumballMonitor {
    pub fn new(gumball_machine: Box<dyn GumballMachineInterface + Send>) -> Self {
        GumballMonitor { gumball_machine }
    }

    pub fn report(&self) {
        println!("📂 Gumball Machine Report");
        println!(
            "📌 Gumball Machine location: {}",
            self.gumball_machine.get_location()
        );
        println!(
            "📌 Number of Gumballs: {}",
            self.gumball_machine.get_count().unwrap()
        );
        println!(
            "📌 Current state: {:?}",
            self.gumball_machine.get_state().unwrap()
        );
        println!("📁 End of Report");
    }
}
