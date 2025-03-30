use std::io::{self};

use crate::physicalio::{data::Data, datatype::Datatype, input::Input};

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

            match input.trim().parse() {
                Ok(n) => return Data::Integer(n),
                _ => (),
            };
        }
    }

    fn data_type(&self) -> crate::physicalio::datatype::Datatype {
        Datatype::ConsoleIn
    }
}
