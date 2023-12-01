use singleton_pattern::choc_o_holic::ChocolateBoiler;

#[test]
fn choc_o_holic_test() {
    println!("======[ choc_o_holic.rs start ]======");
    {
        let mut boiler_1 = ChocolateBoiler::get_boiler().lock().unwrap();
        boiler_1.boil();
        boiler_1.drain();
        boiler_1.fill();
    }
    {
        let mut boiler_2 = ChocolateBoiler::get_boiler().lock().unwrap();
        boiler_2.fill();
        boiler_2.drain();
        boiler_2.boil();
    }
    {
        let mut boiler_3 = ChocolateBoiler::get_boiler().lock().unwrap();
        boiler_3.fill();
        boiler_3.boil();
        boiler_3.drain();
    }
    println!("======[ choc_o_holic.rs end ]======");
}
