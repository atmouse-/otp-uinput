extern crate uinput;

use std::thread;
use std::time::Duration;
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::{Left, Right};
use uinput::event::Event::{Controller, Relative};
use uinput::event::Event::Absolute;
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;

pub struct Uinput {
    device: uinput::Device,
}

impl Uinput {
    pub fn new(mouse_max: Option<(i32, i32)>) -> Self {
        let device = uinput::default().unwrap()
                .name("OTP-MOUSE").unwrap()
                .event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
                .event(uinput::event::Keyboard::All).unwrap()
                .event(Relative(Position(X))).unwrap()
                .event(Relative(Position(Y))).unwrap();
        let device = {
            match mouse_max {
                Some((abs_x_max, abs_y_max)) => {
                    device.event(Absolute(uinput::event::absolute::Absolute::Position(
                        uinput::event::absolute::Position::X
                    ))).unwrap()
                    .max(abs_x_max)
                    .event(Absolute(uinput::event::absolute::Absolute::Position(
                        uinput::event::absolute::Position::Y
                    ))).unwrap()
                    .max(abs_y_max)
                },
                None => device
            }
        }.create().unwrap();

        Uinput {
            device: device,
        }
    }

    pub fn mouse_pos(&mut self, x: i32, y: i32) {
        println!("mousepos {}, {}", x, y);
        self.device.send(X, x).unwrap();
        self.device.send(Y, y).unwrap();
        self.device.synchronize().unwrap();
    }

    pub fn mouse_abs(&mut self, x: i32, y: i32) {
        // TODO: fix absolute action if value same as previous
        println!("mouseabs {}, {}", x, y);
        self.device.send(uinput::event::absolute::Position::X, x).unwrap();
        self.device.send(uinput::event::absolute::Position::Y, y).unwrap();
        self.device.synchronize().unwrap();
    }

    pub fn mouse_click(&mut self) {
        println!("mousein click");
        self.device.write(0x01, 0x110, 1).unwrap();
        self.device.synchronize().unwrap();
        self.device.write(0x01, 0x110, 0).unwrap();
        self.device.synchronize().unwrap();
    }
}