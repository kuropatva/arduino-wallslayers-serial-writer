use std::collections::HashMap;

use crate::physicalio::{data::Data, datatype::Datatype, output::Output, writer::Writer};
pub struct WheelOutWrapper<'a> {
    writer: &'a mut Writer,
}

impl<'a> WheelOutWrapper<'a> {
    pub fn new(writer: &'a mut Writer) -> WheelOutWrapper<'a> {
        WheelOutWrapper { writer }
    }
}

impl<'a> Output for WheelOutWrapper<'a> {
    fn push(&mut self, map: &HashMap<Datatype, crate::physicalio::data::Data>) {
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
