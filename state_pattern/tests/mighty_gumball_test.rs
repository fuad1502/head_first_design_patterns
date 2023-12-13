use state_pattern::mighty_gumball::*;

#[test]
fn mighty_gumball_test() {
    println!("======[ mighty_gumball_test.rs start ]======");
    let mut gumball_machine = GumballMachine::new(0);

    println!("> Try inserting quarter to empty machine");
    gumball_machine.insert_quarter();

    println!("> Refill machine with 10 gumballs");
    gumball_machine.refill(10);

    println!("> Try ejecting quarter");
    gumball_machine.insert_quarter();
    gumball_machine.eject_quarter();

    for _i in 0..10 {
        println!("> Try dispensing gumball");
        gumball_machine.insert_quarter();
        gumball_machine.turn_crank();
    }

    println!("> Try dispensing when out of gumballs");
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();

    println!("> Try refilling again");
    gumball_machine.refill(1);

    println!("> Try dispensing again");
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    println!("======[ mighty_gumball_test.rs end ]======");
}
