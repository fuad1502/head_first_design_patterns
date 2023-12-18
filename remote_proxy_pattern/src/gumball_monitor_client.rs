use remote_proxy_pattern::gumball_machine_stub::GumballMachineStub;
use remote_proxy_pattern::gumball_monitor::GumballMonitor;

#[tokio::main]
async fn main() {
    let gumball_machine_stub = GumballMachineStub::new("http://[::1]:50051".to_string());
    let gumball_monitor = GumballMonitor::new(Box::new(gumball_machine_stub));
    gumball_monitor.report();
}
