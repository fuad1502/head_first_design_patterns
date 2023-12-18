use crate::gumball_machine_service;
use crate::GumballMachineInterface;
use crate::GumballState;
use core::cell::RefCell;
use futures::executor::block_on;
use gumball_machine_service::gumball_machine_service_client::GumballMachineServiceClient;
use gumball_machine_service::{EmptyRequest, RefillRequest};
use tonic::Request;

pub struct GumballMachineStub {
    client: RefCell<GumballMachineServiceClient<tonic::transport::Channel>>,
}

impl GumballMachineStub {
    pub fn new(addr: String) -> Self {
        let client = block_on(GumballMachineServiceClient::connect(addr)).unwrap();
        let client = RefCell::new(client);
        GumballMachineStub { client }
    }
}

impl GumballMachineInterface for GumballMachineStub {
    fn insert_quarter(&mut self) {
        let request = Request::new(EmptyRequest {});
        block_on(self.client.borrow_mut().insert_quarter(request)).unwrap();
    }

    fn eject_quarter(&mut self) {
        let request = Request::new(EmptyRequest {});
        block_on(self.client.borrow_mut().eject_quarter(request)).unwrap();
    }

    fn turn_crank(&mut self) {
        let request = Request::new(EmptyRequest {});
        block_on(self.client.borrow_mut().turn_crank(request)).unwrap();
    }

    fn refill(&mut self, gumballs: usize) {
        let request = Request::new(RefillRequest {
            gumballs: Some(gumballs as u32),
        });
        block_on(self.client.borrow_mut().refill(request)).unwrap();
    }

    fn get_state(&self) -> Option<GumballState> {
        let request = Request::new(EmptyRequest {});
        let response = block_on(self.client.borrow_mut().get_state(request)).unwrap();
        if let Some(state) = response.into_inner().state {
            match state.try_into().unwrap() {
                gumball_machine_service::GumballState::NoQuarter => Some(GumballState::NoQuarter),
                gumball_machine_service::GumballState::HasQuarter => Some(GumballState::HasQuarter),
                gumball_machine_service::GumballState::GumballSold => {
                    Some(GumballState::GumballSold)
                }
                gumball_machine_service::GumballState::OutOfGumballs => {
                    Some(GumballState::OutOfGumballs)
                }
                gumball_machine_service::GumballState::Winner => Some(GumballState::Winner),
            }
        } else {
            None
        }
    }

    fn get_count(&self) -> Option<usize> {
        let request = Request::new(EmptyRequest {});
        let response = block_on(self.client.borrow_mut().get_gumball_count(request)).unwrap();
        if let Some(count) = response.into_inner().count {
            Some(count as usize)
        } else {
            None
        }
    }

    fn get_location(&self) -> String {
        let request = Request::new(EmptyRequest {});
        let response = block_on(self.client.borrow_mut().get_location(request)).unwrap();
        response.into_inner().location.unwrap()
    }
}
