use super::FlyBehaviour;

// Implement various FlyBehaviours
pub struct FlyWithWings {}
impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("Fly!");
    }
}
pub struct FlyNoWay {}
impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("...");
    }
}
pub struct FlyRocketShip {}
impl FlyBehaviour for FlyRocketShip {
    fn fly(&self) {
        println!("Lift off! 🚀");
    }
}
