use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::{
    fs::File,
    io::{self, Read},
};

use crate::raw_mode_guard::RawModeGuard;

pub struct Editor {
    pub(crate) cursor_x: usize,
    pub(crate) cursor_y: usize,
    pub(crate) lines: Vec<String>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            lines: vec![String::new()],
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let _raw_mode = RawModeGuard::new()?;
        self.clear_screen()?;

        for (y, line) in self.lines.iter().enumerate() {
            queue!(
                io::stdout(),
                MoveTo(0, y as u16),
                Clear(ClearType::CurrentLine),
                Print(line),
            )?;
        }
        execute!(io::stdout(), MoveTo(0, 0))?;

        loop {
            if let Event::Key(event) = read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers,
                        ..
                    } if modifiers.is_empty() || modifiers == KeyModifiers::SHIFT => {
                        self.handle_char(c)?;
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => self.move_left()?,
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => self.move_right()?,
                    KeyEvent {
                        code: KeyCode::Up, ..
                    } => self.move_up()?,
                    KeyEvent {
                        code: KeyCode::Down,
                        ..
                    } => self.move_down()?,
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers,
                        ..
                    } => self.handle_backspace(modifiers.intersects(KeyModifiers::CONTROL))?,
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => self.handle_enter()?,
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => break,
                    _ => {}
                }
            }
        }
        self.clear_screen()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        self.cursor_x = 0;
        self.cursor_y = 0;
        Ok(())
    }

    pub fn load_file(&mut self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;
        self.lines.clear();

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.lines = contents.lines().map(String::from).collect();

        Ok(())
    }
}
