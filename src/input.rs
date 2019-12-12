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
            33 => {
                // !
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_1).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            34 => {
                // "
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Apostrophe).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            35 => {
                // #
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_3).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            36 => {
                // $
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_4).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            37 => {
                // %
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_5).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            38 => {
                // &
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_7).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            39 => {self.device.click(&keyboard::Key::Apostrophe).unwrap()}, /* ' */
            40 => {
                // (
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_9).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            41 => {
                // )
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_0).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            42 => {
                // *
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_8).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            43 => {
                // +
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Equal).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            44 => {
                // ,
                self.device.click(&keyboard::Key::Comma).unwrap();
            },
            45 => {self.device.click(&keyboard::Key::Minus).unwrap()}, /* - */
            46 => {self.device.click(&keyboard::Key::Dot).unwrap()}, /* . */
            47 => {self.device.click(&keyboard::Key::Slash).unwrap()}, /* / */
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
            58 => {
                // :
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::SemiColon).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            59 => {self.device.click(&keyboard::Key::SemiColon).unwrap()}, /* ; */
            60 => {
                // <
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Comma).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            61 => {self.device.click(&keyboard::Key::Equal).unwrap()}, /* = */
            62 => {
                // >
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Dot).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            63 => {
                // ?
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Slash).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            64 => {
                // @
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_2).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            65 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::A).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            66 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::B).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            67 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::C).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            68 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::D).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            69 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::E).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            70 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::F).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            71 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::G).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            72 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::H).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            73 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::I).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            74 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::J).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            75 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::K).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            76 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::L).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            77 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::M).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            78 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::N).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            79 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::O).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            80 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::P).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            81 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Q).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            82 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::R).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            83 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::S).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            84 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::T).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            85 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::U).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            86 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::V).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            87 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::W).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            88 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::X).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            89 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Y).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            90 => {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Z).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            91 => {self.device.click(&keyboard::Key::LeftBrace).unwrap()}, /* [ */
            92 => {self.device.click(&keyboard::Key::BackSlash).unwrap()}, /* \ */
            93 => {self.device.click(&keyboard::Key::RightBrace).unwrap()}, /* ] */
            94 => {
                // ^
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::_6).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            95 => {
                // _
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Minus).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            96 => {self.device.click(&keyboard::Key::Grave).unwrap()}, /* ` */
            97 => {self.device.click(&keyboard::Key::A).unwrap()},
            98 => {self.device.click(&keyboard::Key::B).unwrap()},
            99 => {self.device.click(&keyboard::Key::C).unwrap()},
            100 => {self.device.click(&keyboard::Key::D).unwrap()},
            101 => {self.device.click(&keyboard::Key::E).unwrap()},
            102 => {self.device.click(&keyboard::Key::F).unwrap()},
            103 => {self.device.click(&keyboard::Key::G).unwrap()},
            104 => {self.device.click(&keyboard::Key::H).unwrap()},
            105 => {self.device.click(&keyboard::Key::I).unwrap()},
            106 => {self.device.click(&keyboard::Key::J).unwrap()},
            107 => {self.device.click(&keyboard::Key::K).unwrap()},
            108 => {self.device.click(&keyboard::Key::L).unwrap()},
            109 => {self.device.click(&keyboard::Key::M).unwrap()},
            110 => {self.device.click(&keyboard::Key::N).unwrap()},
            111 => {self.device.click(&keyboard::Key::O).unwrap()},
            112 => {self.device.click(&keyboard::Key::P).unwrap()},
            113 => {self.device.click(&keyboard::Key::Q).unwrap()},
            114 => {self.device.click(&keyboard::Key::R).unwrap()},
            115 => {self.device.click(&keyboard::Key::S).unwrap()},
            116 => {self.device.click(&keyboard::Key::T).unwrap()},
            117 => {self.device.click(&keyboard::Key::U).unwrap()},
            118 => {self.device.click(&keyboard::Key::V).unwrap()},
            119 => {self.device.click(&keyboard::Key::W).unwrap()},
            120 => {self.device.click(&keyboard::Key::X).unwrap()},
            121 => {self.device.click(&keyboard::Key::Y).unwrap()},
            122 => {self.device.click(&keyboard::Key::Z).unwrap()},
            123 => {
                // {
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::LeftBrace).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            124 => {
                // |
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::BackSlash).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            125 => {
                // }
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::RightBrace).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
            126 => {
                // ~
                self.device.press(&keyboard::Key::LeftShift).unwrap();
                self.device.click(&keyboard::Key::Grave).unwrap();
                self.device.release(&keyboard::Key::LeftShift).unwrap();
            },
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