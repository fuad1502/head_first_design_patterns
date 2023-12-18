use gumball_machine_service::gumball_machine_service_server::GumballMachineServiceServer;
use remote_proxy_pattern::gumball_machine::GumballMachine;
use remote_proxy_pattern::gumball_machine_service;
use remote_proxy_pattern::gumball_machine_skeleton::GumballMachineSkeleton;
use remote_proxy_pattern::gumball_machine_stub::GumballMachineStub;
use remote_proxy_pattern::gumball_monitor::GumballMonitor;
use remote_proxy_pattern::GumballMachineInterface;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use termion;
use tokio::task;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the gumball machine
    let gumball_machine = GumballMachine::new(5, "Casa 39".to_string());
    let gumball_machine = Arc::new(Mutex::new(gumball_machine));
    let gumball_machine_clone = gumball_machine.clone();

    // Start the gumball machine server
    let _server_task = task::spawn(async move {
        let addr = "[::1]:50051".parse().unwrap();
        let gumball_machine_skeleton = GumballMachineSkeleton::new(gumball_machine_clone);
        let _ = Server::builder()
            .add_service(GumballMachineServiceServer::new(gumball_machine_skeleton))
            .serve(addr)
            .await;
    });

    // Wait for the server to start
    thread::sleep(std::time::Duration::from_secs(1));

    // Print mutex
    let print_mutex = Arc::new(Mutex::new(()));
    let print_mutex_clone = Arc::clone(&print_mutex);

    // Print the demo title
    clear_screen();
    println!(
        "
        REMOTE PROXY PATTERN DEMO    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

     ⠀⠀⠀⠀⠀⠀⣿⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣿⡿⠿⢿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠀⢠⣤⠀⠀⣴⠀⠀⠀⠀⠀⣿⠀⣶⠀⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠰⠾⠿⠶⠾⠿⠶⠶⠶⠶⠀⣿⣀⣉⣀⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⢀⣤⡀⠀⠀⣿⣏⣉⣹⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠐⠒⠒⠒⠒⠒⠚⠛⠓⠒⠀⣿⣯⣉⣹⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠀⢠⡀⠀⣾⠀⠀⣶⡆⠀⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠘⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠀⣶⣦⠀⣶⣶⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣏⣉⣹⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⠶⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⠀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⡄⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⠀⠀⣿⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀

    GUMBALL MACHINE REMOTE MONITORING"
    );

    // Create the gumball machine stub
    // INFO: Somehow I cannot create the gumball machine stub in the same thread as the gumball monitor
    let gumball_machine_stub = GumballMachineStub::new("http://[::1]:50051".to_string());

    // Create gumball monitor thread
    let _monitor_thread = thread::spawn(move || {
        let gumball_monitor = GumballMonitor::new(Box::new(gumball_machine_stub));
        loop {
            let p = print_mutex.lock().unwrap();
            move_cursor_up();
            gumball_monitor.report();
            drop(p);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Create thread for playing with actual gumball_machine
    let play_thread = thread::spawn(move || {
        // Refill gumball machine
        let p = print_mutex_clone.lock().unwrap();
        let mut gm = gumball_machine.lock().unwrap();
        move_cursor_down();
        gm.refill(10);
        drop(gm);
        drop(p);
        thread::sleep(std::time::Duration::from_secs(1));
        for _i in 0..10 {
            // Insert quarter
            let p = print_mutex_clone.lock().unwrap();
            let mut gm = gumball_machine.lock().unwrap();
            move_cursor_down();
            gm.insert_quarter();
            drop(gm);
            drop(p);
            thread::sleep(std::time::Duration::from_secs(1));

            // Turn crank
            let p = print_mutex_clone.lock().unwrap();
            let mut gm = gumball_machine.lock().unwrap();
            move_cursor_down();
            gm.turn_crank();
            drop(gm);
            drop(p);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    play_thread.join().unwrap();

    Ok(())
}

fn clear_screen() {
    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    );
}

fn move_cursor_up() {
    print!("{}", termion::cursor::Goto(1, 20));
    for _i in 0..5 {
        print!("{}", termion::clear::CurrentLine);
        print!("{}", termion::cursor::Down(1));
    }
    print!("{}", termion::cursor::Goto(1, 20));
}

fn move_cursor_down() {
    print!(
        "{}{}",
        termion::cursor::Goto(1, 26),
        termion::clear::AfterCursor
    );
    println!("🔧 Action Performed: ");
}
