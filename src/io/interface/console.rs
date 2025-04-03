use std::io::{self};

use crate::data::{data::Data, datatype::Datatype};
use crate::io::input::Input;

pub struct Console {}

impl Console {
    pub fn new() -> Console {
        Console {}
    }
}

impl Input for Console {
    fn get(&self) -> Data {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Incorrect value");

            if let Ok(n) = input.trim().parse() {
                return Data::Integer(n);
            };
        }
    }

    fn data_type(&self) -> Datatype {
        Datatype::ConsoleIn
    }
}
