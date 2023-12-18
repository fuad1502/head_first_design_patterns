use crate::gumball_machine_service;
use crate::GumballMachine;
use crate::GumballMachineInterface;
use crate::GumballState;
use gumball_machine_service::gumball_machine_service_server::GumballMachineService;
use gumball_machine_service::{
    EmptyRequest, EmptyResponse, GumballCountResponse, GumballLocationResponse,
    GumballStateResponse, RefillRequest,
};
use std::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct GumballMachineSkeleton {
    gumball_machine: Mutex<GumballMachine>,
}

impl GumballMachineSkeleton {
    pub fn new(gumballs: usize, location: String) -> Self {
        GumballMachineSkeleton {
            gumball_machine: Mutex::new(GumballMachine::new(gumballs, location)),
        }
    }
}

#[tonic::async_trait]
impl GumballMachineService for GumballMachineSkeleton {
    async fn get_gumball_count(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GumballCountResponse>, Status> {
        let count = match self.gumball_machine.lock().unwrap().get_count() {
            Some(count) => Some(count as u32),
            None => None,
        };
        let response = GumballCountResponse { count };
        Ok(Response::new(response))
    }

    async fn get_state(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GumballStateResponse>, Status> {
        let state = match self.gumball_machine.lock().unwrap().get_state() {
            Some(state) => match state {
                GumballState::NoQuarter => {
                    Some(gumball_machine_service::GumballState::NoQuarter as i32)
                }
                GumballState::HasQuarter => {
                    Some(gumball_machine_service::GumballState::HasQuarter as i32)
                }
                GumballState::GumballSold => {
                    Some(gumball_machine_service::GumballState::GumballSold as i32)
                }
                GumballState::OutOfGumballs => {
                    Some(gumball_machine_service::GumballState::OutOfGumballs as i32)
                }
                GumballState::Winner => Some(gumball_machine_service::GumballState::Winner as i32),
            },
            None => None,
        };
        let response = GumballStateResponse { state };
        Ok(Response::new(response))
    }

    async fn refill(
        &self,
        request: Request<RefillRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let gumballs = match request.into_inner().gumballs {
            Some(gumballs) => gumballs as usize,
            None => 0,
        };
        self.gumball_machine.lock().unwrap().refill(gumballs);
        Ok(Response::new(EmptyResponse {}))
    }

    async fn insert_quarter(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.gumball_machine.lock().unwrap().insert_quarter();
        Ok(Response::new(EmptyResponse {}))
    }

    async fn eject_quarter(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.gumball_machine.lock().unwrap().eject_quarter();
        Ok(Response::new(EmptyResponse {}))
    }

    async fn turn_crank(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.gumball_machine.lock().unwrap().turn_crank();
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_location(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GumballLocationResponse>, Status> {
        let location = Some(self.gumball_machine.lock().unwrap().get_location());
        let response = GumballLocationResponse { location };
        Ok(Response::new(response))
    }
}
