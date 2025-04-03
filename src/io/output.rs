use std::collections::HashMap;

use crate::data::{data::Data, datatype::Datatype};
pub trait Output {
    fn push(&mut self, map: &HashMap<Datatype, Data>);
}
