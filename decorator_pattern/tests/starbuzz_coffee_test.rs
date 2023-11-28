use decorator_pattern::starbuzz_coffee::{beverages, condiments, Beverage, BeverageSize, CondimentDecorator};

#[test]
fn starbuzz_coffee_test() {
    println!("======[ starbuzz_coffee.rs start ]======");
    // Order Espresso
    let beverage_1: Box<dyn Beverage> = Box::new(beverages::Espresso::new(BeverageSize::VENTI));
    println!(
        "Ordered: {}; Size: {:?}; Cost: {}",
        beverage_1.description(),
        beverage_1.get_size(),
        beverage_1.cost()
    );
    // Order Dark Roast with Double Mocha and a Whip
    let mut beverage_2: Box<dyn Beverage> = Box::new(beverages::DarkRoast::new(BeverageSize::TALL));
    beverage_2 = condiments::Mocha::decorate(beverage_2);
    beverage_2 = condiments::Mocha::decorate(beverage_2);
    beverage_2 = condiments::Whip::decorate(beverage_2);
    println!(
        "Ordered: {}; Size: {:?}; Cost: {}",
        beverage_2.description(),
        beverage_2.get_size(),
        beverage_2.cost()
    );
    // Order House Blend with Soy, Mocha, and Whip
    let mut beverage_3: Box<dyn Beverage> = Box::new(beverages::HouseBlend::new(BeverageSize::TALL));
    beverage_3 = condiments::Soy::decorate(beverage_3);
    beverage_3 = condiments::Mocha::decorate(beverage_3);
    beverage_3 = condiments::Whip::decorate(beverage_3);
    println!(
        "Ordered: {}; Size: {:?}; Cost: {}",
        beverage_3.description(),
        beverage_3.get_size(),
        beverage_3.cost()
    );
    println!("======[ starbuzz_coffee.rs end ]======");
}
