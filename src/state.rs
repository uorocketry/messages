use defmt::{info, Format};
use messages_proc_macros_lib::common_derives;
use core::fmt::Debug; 
use smlang::statemachine;

#[common_derives]
pub enum DeviceState {
    Init,
    Idle,
    Calibration,
    Discovery,
    Recovery,
    Collection,    
    Processing,
    Fault,
}