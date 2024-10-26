use messages_proc_macros_lib::common_derives;

#[common_derives]
#[derive(Copy)]
pub enum Node {
    PressureBoard,
    TemperatureBoard,
    StrainBoard,
}

impl From<Node> for u16 {
    fn from(node: Node) -> Self {
        match node {
            Node::PressureBoard => 0,
            Node::TemperatureBoard => 1,
            Node::StrainBoard => 2,
        }
    }
}
