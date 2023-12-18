pub mod gumball_machine;
pub mod gumball_machine_skeleton;
pub mod gumball_machine_stub;
pub mod gumball_monitor;
pub mod gumball_machine_service {
    tonic::include_proto!("gumball_machine_service");
}

pub use gumball_machine::GumballMachine;
pub use gumball_machine::GumballMachineInterface;
pub use gumball_machine::GumballState;
