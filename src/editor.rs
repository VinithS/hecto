use std::io::{self, Write};

use termion::{event::Key, input::TermRead};

pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { quit: false }
    }

    pub fn run(&mut self) {
        loop {
            println!("loopoing \r");
            if let Err(e) = self.refresh_screen() {
                Editor::die(&e);
            }

            if self.quit {
                println!("breaking \r");
                break;
            }

            if let Err(e) = self.execute_key() {
                Editor::die(&e);
            }
        }
    }

    fn execute_key(&mut self) -> Result<(), std::io::Error> {
        let pressed = read_keys()?;

        println!("pressed, {pressed:?} \r");

        match pressed {
            Key::Char(c) => println!("{c} \r"),
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if self.quit {
            println!("exiting hecto.. goodbye.\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }

    fn die(e: &std::io::Error) {
        print!("{}", termion::clear::All);
        panic!("{}", e);
    }
}

fn read_keys() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
        println!("only supported key pressed for now.. \n");
    }
}
