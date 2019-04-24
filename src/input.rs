extern crate uinput;

use std::thread;
use std::time::Duration;
use uinput::event::keyboard;

pub struct Uinput {
    device: uinput::Device,
}

impl Uinput {
    pub fn new() -> Self {
        let device = uinput::default().unwrap()
                .name("OTP-INPUT").unwrap()
                .event(uinput::event::Keyboard::All).unwrap()
                .create().unwrap();
        Uinput {
            device: device,
        }
    }

    pub fn keyin(&mut self, key: u8) {
        println!("keyin {}", key);
        match key {
            48 => {self.device.click(&keyboard::Key::_0).unwrap()},
            49 => {self.device.click(&keyboard::Key::_1).unwrap()},
            50 => {self.device.click(&keyboard::Key::_2).unwrap()},
            51 => {self.device.click(&keyboard::Key::_3).unwrap()},
            52 => {self.device.click(&keyboard::Key::_4).unwrap()},
            53 => {self.device.click(&keyboard::Key::_5).unwrap()},
            54 => {self.device.click(&keyboard::Key::_6).unwrap()},
            55 => {self.device.click(&keyboard::Key::_7).unwrap()},
            56 => {self.device.click(&keyboard::Key::_8).unwrap()},
            57 => {self.device.click(&keyboard::Key::_9).unwrap()},
            _ => {return},
        }
        self.device.synchronize().unwrap();
    }

    fn loop_inputs(rx: std::sync::mpsc::Receiver<u8>) {
        thread::sleep(Duration::from_secs(1));

        loop {
            let key = rx.recv().unwrap();
            println!("recv {}", key);
            // device.click(&keyboard::Key::O).unwrap();
            // device.synchronize().unwrap();
        }
    }
}