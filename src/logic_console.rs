use std::collections::HashMap;

use crate::{
    logic::Logic,
    physicalio::{data::Data, datatype::Datatype},
};

pub struct LogicBasicConsole {}

impl LogicBasicConsole {
    pub fn new() -> LogicBasicConsole {
        LogicBasicConsole {}
    }
}

impl Logic for LogicBasicConsole {
    fn process(&self, input_map: &HashMap<Datatype, Data>) -> HashMap<Datatype, Data> {
        let mut output_map = HashMap::new();

        if let Some(val) = input_map.get(&Datatype::ConsoleIn) {
            output_map.insert(Datatype::WheelOut, val.clone());
        }

        output_map
    }
}
