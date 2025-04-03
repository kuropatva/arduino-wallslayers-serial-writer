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

pub struct Writer {
    port: COMPort,
}

impl Writer {
    pub fn new(serial_port_name: &str) -> Writer {
        let mut port = serial::open(serial_port_name).unwrap();
        port.configure(&SETTINGS).unwrap();
        port.set_timeout(Duration::from_secs(30)).unwrap();

        println!("Waiting for arduino connection...");

        loop {
            let order = Order::HELLO as i8;
            write_i8(&mut port, order).unwrap();
            let mut answer = read_i8(&mut port).unwrap();
            print!("'{}'", answer);

            if answer > 6 {
                // testing purpose, "0" = 48
                answer -= 48;
            }

            let received_order = Order::from_i8(answer);
            if received_order.is_some() {
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
        let _ = write_i32(&mut self.port, i);
    }

    pub fn write16(&mut self, i: i16) {
        let _ = write_i16(&mut self.port, i);
    }
}
