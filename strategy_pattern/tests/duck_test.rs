use strategy_pattern::duck::{self, Duck};

#[test]
fn main() {
    println!("======[ duck_test.rs start ]======");
    let duck_1 = duck::MallardDuck::new();
    let mut duck_2 = duck::RubberDuck::new();

    duck_1.display();
    duck_1.quack();
    duck_1.fly();

    duck_2.display();
    duck_2.quack();
    duck_2.fly();

    duck_2.set_fly_behaviour(Box::new(duck::fly_behaviours::FlyRocketShip {}));
    duck_2.fly();
    println!("======[ duck_test.rs end ]======");
}
