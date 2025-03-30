use std::{thread, time::Duration};

use robust_arduino_serial::{Order, read_i8, write_i8, write_i16, write_i32};
use serial::{SerialPort, windows::COMPort};

extern crate robust_arduino_serial;
extern crate serial;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

const SERIAL_PORT: &str = "COM10";

pub struct Writer {
    port: COMPort,
}

impl Writer {
    pub fn new() -> Writer {
        let mut port = serial::open(&SERIAL_PORT.to_string()).unwrap();
        port.configure(&SETTINGS).unwrap();
        port.set_timeout(Duration::from_secs(30)).unwrap();

        println!("Waiting for arduino connection...");

        loop {
            let order = Order::HELLO as i8;
            write_i8(&mut port, order).unwrap();
            let mut a = read_i8(&mut port).unwrap();
            print!("'{}'", a);
            if (a > 6) {
                a -= 48;
            }

            let received_order = Order::from_i8(a);
            if received_order != None {
                let received_order = received_order.unwrap();
                if received_order == Order::ALREADY_CONNECTED {
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }

        println!("Connected!");

        Writer { port }
    }

    pub fn write32(&mut self, i: i32) {
        write_i32(&mut self.port, i);
    }

    pub fn write16(&mut self, i: i16) {
        write_i16(&mut self.port, i);
    }
}
