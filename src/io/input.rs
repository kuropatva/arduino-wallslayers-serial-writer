use crate::data::{data::Data, datatype::Datatype};

pub trait Input {
    fn get(&self) -> Data;
    fn data_type(&self) -> Datatype;
}
