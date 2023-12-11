use template_pattern::caffein_beverage::*;

#[test]
fn caffein_beverage_test() {
    println!("======[ caffein_beverage_test.rs start ]======");
    let coffee = Coffee::new();
    coffee.prepare_recipe();
    println!("======[ caffein_beverage_test.rs end ]======");
}
