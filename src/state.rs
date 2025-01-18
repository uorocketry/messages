use messages_proc_macros_lib::common_derives;

// #[common_derives]
// pub struct State {
//     pub data: StateData,
// }

#[common_derives]
pub enum State {
    Init,
    WaitForTakeoff,
    Ascent,
    Descent,
    TerminalDescent,
    WaitForRecovery,
    Abort,
    Collection,
    Calibration,
    Discovery,
    Idle,
}

// impl State {
//     pub fn new(data: impl Into<StateData>) -> Self {
//         State { data: data.into() }
//     }
// }
