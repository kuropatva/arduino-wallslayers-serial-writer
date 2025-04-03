use std::collections::HashMap;

use crate::data::{data::Data, datatype::Datatype};
pub trait Logic {
    fn process(&self, input_map: &HashMap<Datatype, Data>) -> HashMap<Datatype, Data>;
}
