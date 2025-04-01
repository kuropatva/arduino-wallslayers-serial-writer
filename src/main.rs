use std::collections::HashMap;

use interface::console::Console;
use logic::Logic;
use logic_console::LogicBasicConsole;
use physicalio::{
    data::Data, datatype::Datatype, electronics::wheel_out_wrapper::WheelOutWrapper, input::Input,
    output::Output, writer::Writer,
};

mod interface;
pub mod logic;
pub mod logic_console;
mod physicalio;

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
