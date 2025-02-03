use crate::node::Node;
use derive_more::From;
use messages_proc_macros_lib::common_derives;

#[common_derives]
#[derive(From)]
pub enum Command {
    DeployDrogue(DeployDrogue),
    DeployMain(DeployMain),
    PowerDown(PowerDown),
    RadioRateChange(RadioRateChange),
    Online(Online),
}

#[common_derives]
pub struct Online {
    pub online: bool,
}

#[common_derives]
#[derive(From)]
pub struct DeployDrogue {
    pub val: bool,
}

#[common_derives]
#[derive(From)]
pub struct DeployMain {
    pub val: bool,
    // Auth?
}

#[common_derives]
#[derive(From)]
pub struct PowerDown {
    pub board: Node,
}

#[common_derives]
#[derive(From)]
pub struct RadioRateChange {
    pub rate: RadioRate,
}

#[common_derives]
#[derive(From)]
pub enum RadioRate {
    Fast,
    Slow,
}
