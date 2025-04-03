use std::collections::HashMap;

use crate::io::output::Output;
use crate::io::physical::writer::Writer;

use crate::data::{data::Data, datatype::Datatype};

pub struct WheelOutWrapper<'a> {
    writer: &'a mut Writer,
}

impl<'a> WheelOutWrapper<'a> {
    pub fn new(writer: &'a mut Writer) -> WheelOutWrapper<'a> {
        WheelOutWrapper { writer }
    }
}

impl Output for WheelOutWrapper<'_> {
    fn push(&mut self, map: &HashMap<Datatype, Data>) {
        let Some(data) = &map.get(&Datatype::WheelOut) else {
            return;
        };
        let Data::Integer(data) = data else {
            return;
        };
        println!("{}", data);
        self.writer.write32(*data);
    }
}
