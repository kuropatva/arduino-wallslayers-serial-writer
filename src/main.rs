use std::collections::HashMap;

use interface::console::Console;
use physicalio::{
    data::Data, datatype::Datatype, electronics::wheel_out_wrapper::WheelOutWrapper, input::Input,
    output::Output, writer::Writer,
};

mod interface;
mod physicalio;

fn main() -> ! {
    let mut writer = Writer::new();
    let mut wrapper = WheelOutWrapper::new(&mut writer);
    let console = Console::new();
    let mut map: HashMap<Datatype, Data> = HashMap::new();
    loop {
        map.clear();
        map.insert(Datatype::WheelOut, console.get());
        wrapper.push(&map);
    }
}
