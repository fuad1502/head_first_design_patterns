use super::QuackBehaviour;

// Implement various QuackBehaviours
pub struct Quack {}
pub struct Squeak {}
pub struct MuteQuack {}
impl QuackBehaviour for Quack {
    fn quack(&self) {
        println!("Quack!");
    }
}
impl QuackBehaviour for Squeak {
    fn quack(&self) {
        println!("Squeak!");
    }
}
impl QuackBehaviour for MuteQuack {
    fn quack(&self) {
        println!("...");
    }
}
