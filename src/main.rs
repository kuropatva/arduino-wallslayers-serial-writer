use std::collections::HashMap;

use crate::data::{data::Data, datatype::Datatype};
use crate::io::interface::console::Console;
use io::physical::{electronics::wheel_out_wrapper::WheelOutWrapper, writer::Writer};
use io::{input::Input, output::Output};
use processing::{logic::Logic, logic_console::LogicBasicConsole};

pub mod data;
pub mod io;
pub mod processing;

fn main() -> ! {
    let serial_port_name = "COM10";

    let mut writer = Writer::new(serial_port_name);

    let mut inputs: Vec<Box<dyn Input>> = Vec::new();
    inputs.push(Box::new(Console::new()));

    let mut outputs: Vec<Box<dyn Output>> = Vec::new();
    outputs.push(Box::new(WheelOutWrapper::new(&mut writer)));

    let logic: Box<dyn Logic> = Box::new(LogicBasicConsole::new());

    let mut map: HashMap<Datatype, Data> = HashMap::new();

    loop {
        let in_map = HashMap::new();

        for i in &inputs {
            map.insert(i.data_type(), i.get());
        }

        let out_map = logic.process(&in_map);

        outputs.iter_mut().for_each(|i| i.push(&out_map));
    }
}
