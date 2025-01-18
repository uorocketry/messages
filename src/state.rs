use defmt::{info, Format};
use messages_proc_macros_lib::common_derives;
use core::fmt::Debug; 


#[enum_dispatch::enum_dispatch]
pub trait State: Debug {
    fn enter(&self, _context: &mut StateMachineContext)
    where
        Self: Format,
    {
        info!("Enter {:?}", self)
    }
    fn exit(&self)
    where
        Self: Format,
    {
        info!("Exit {:?}", self)
    }
    fn event(&mut self, _event: DeviceState) -> Option<DeviceState> {
        None
    }
    fn step(&mut self, context: &mut StateMachineContext) -> Option<DeviceState>;
}

#[common_derives]
pub struct Initializing {
    pub data: i32,
}

#[common_derives]
pub struct WaitForTakeoff {
    pub data: i32,
}

#[enum_dispatch::enum_dispatch(State)]
#[derive(Format, Debug)]
#[common_derives]
pub enum DeviceState {
    Initializing(Initializing),
    WaitForTakeoff(WaitForTakeoff),

}

// impl State {
//     pub fn new(data: impl Into<StateData>) -> Self {
//         State { data: data.into() }
//     }
// }
