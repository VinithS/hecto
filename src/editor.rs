use std::io::{self, stdout};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        let _a = stdout().into_raw_mode().unwrap();

        for e in io::stdin().events() {
            match e {
                Ok(e) => match e {
                    termion::event::Event::Key(k) => match k {
                        Key::Char(c) => {
                            println!("{}\r", c)
                        }
                        Key::Ctrl('q') => {
                            println!("exiting..\r");
                            break;
                        }
                        _ => println!("{:?}\r", e),
                    },
                    termion::event::Event::Mouse(m) => println!("Mouse event todo: {:?}\r", m),
                    termion::event::Event::Unsupported(u) => {
                        println!("Unsupported event todo: {:?}\r", u)
                    }
                },
                Err(err) => die(err),
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
