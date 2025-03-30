use std::collections::HashMap;

use super::data::Data;
use super::datatype::Datatype;
pub trait Output {
    fn push(&mut self, map: &HashMap<Datatype, Data>);
}
