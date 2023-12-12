use iterator_composite_pattern::objectville_restaurant::*;

#[test]
fn objectville_restaurant_test() {
    println!("======[ objectville_restaurant_test.rs start ]======");
    let mut all_menu = MenuComponent::new_composite("ALL MENU", "All menus combined");

    let mut pancake_house_menu = MenuComponent::new_composite("PANCAKE HOUSE MENU", "Breakfast");
    let mut diner_menu = MenuComponent::new_composite("DINER MENU", "Lunch");
    let mut cafe_menu = MenuComponent::new_composite("CAFE MENU", "Dinner");
    let mut dessert_menu = MenuComponent::new_composite("DESSERT MENU", "Dessert of course");

    pancake_house_menu
        .add(MenuComponent::new_leaf(
            "Regular Pancake Breakfast",
            "Pancake with fried eggs, sausage",
            false,
            2.99,
        ))
        .unwrap();
    diner_menu
        .add(MenuComponent::new_leaf(
            "Vegeterian BLT",
            "(Fakin') Bacon with lettuce & tomato on whole wheat",
            true,
            2.99,
        ))
        .unwrap();
    diner_menu
        .add(MenuComponent::new_leaf(
            "Pasta",
            "Spaghetti with Marinara Sauce, and a slice of sourdough bread",
            true,
            3.89,
        ))
        .unwrap();
    dessert_menu
        .add(MenuComponent::new_leaf(
            "Apple Pie",
            "Apple pie with a flakey crust, topped with vanilla ice cream",
            true,
            1.59,
        ))
        .unwrap();
    cafe_menu
        .add(MenuComponent::new_leaf(
            "Soup of the day",
            "A cup of the soup of the day, with a side salad",
            false,
            3.69,
        ))
        .unwrap();
    diner_menu.add(dessert_menu).unwrap();
    all_menu.add(pancake_house_menu).unwrap();
    all_menu.add(diner_menu).unwrap();
    all_menu.add(cafe_menu).unwrap();

    let waitterss = Waittress::new(all_menu);
    waitterss.print();
    println!("======[ objectville_restaurant_test.rs end ]======");
}
