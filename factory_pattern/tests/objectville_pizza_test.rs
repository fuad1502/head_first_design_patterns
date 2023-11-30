use factory_pattern::objectville_pizza::{
    pizza_store::{CaliforniaPizzaStore, NYPizzaStore},
    *,
};

#[test]
fn objectville_pizza_test() {
    println!("======[ objectville_pizza_test.rs start ]======");
    let pizza_store = NYPizzaStore::new();
    let _pizza = pizza_store.order_pizza("cheese");
    let pizza_store = CaliforniaPizzaStore::new();
    let _pizza = pizza_store.order_pizza("clam");
    println!("======[ objectville_pizza_test.rs end ]======");
}
