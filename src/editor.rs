use crate::terminal::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position(pub usize, pub usize);

pub struct Editor {
    quit: bool,
    cursor_pos: Position,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            quit: false,
            cursor_pos: Position(0, 0),
            terminal: Terminal::new().expect("Failed to initialize terminal."),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                Editor::die(&e);
            }
            if self.quit {
                break;
            }
            if let Err(e) = self.execute_key() {
                Editor::die(&e);
            }
        }
    }

    fn execute_key(&mut self) -> Result<(), std::io::Error> {
        let pressed = Terminal::read_key()?;
        match pressed {
            // Key::Char(c) => println!("{c}\r"),
            Key::Ctrl('q') => self.quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed),
            _ => println!("huh?\r"),
        }

        Ok(())
    }

    // y grown down
    fn move_cursor(&mut self, key: Key) {
        let mut x: usize = self.cursor_pos.0;
        let mut y: usize = self.cursor_pos.1;

        let size = self.terminal.size();
        let h = size.height.saturating_sub(1) as usize;
        let w = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < h {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < w {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = h,
            Key::End => x = w,
            Key::Home => x = 0,
            _ => (),
        }
        self.cursor_pos = Position(x, y);
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position(0, 0));

        if self.quit {
            Terminal::clear_screen();
            println!("exiting hecto.. goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for r in 0..height - 1 {
            Terminal::clear_current_line();
            if r == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_msg = format!("Hecto editor -- version {VERSION}\r");
        let welcome_msg_len = welcome_msg.len();
        let width = self.terminal.size().width as usize;
        let padding = width.saturating_sub(welcome_msg_len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        println!("{welcome_msg}\r");
    }

    fn die(e: &std::io::Error) {
        Terminal::clear_screen();
        panic!("{e}");
    }
}
