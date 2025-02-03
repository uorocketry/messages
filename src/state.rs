use messages_proc_macros_lib::common_derives;

#[common_derives]
pub enum State {
    Initializing,
    WaitForTakeoff,
    Ascent,
    Descent,
    TerminalDescent,
    WaitForRecovery,
    Abort,
    Idle,
    Calibration,
    Processing,
    Fault,
    Recovery,
    Collection, 
}
