use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::{
    cmp::min,
    io::{self},
};

use crate::editor::Editor;

impl Editor {
    pub(crate) fn handle_char(&mut self, c: char) -> io::Result<()> {
        self.lines[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;

        execute!(
            io::stdout(),
            Clear(ClearType::UntilNewLine),
            MoveTo(self.cursor_x as u16 - 1, self.cursor_y as u16),
            Print(&self.lines[self.cursor_y][self.cursor_x - 1..]),
            MoveTo(self.cursor_x as u16, self.cursor_y as u16),
        )?;

        Ok(())
    }

    pub(crate) fn handle_backspace(&mut self, ctrl: bool) -> io::Result<()> {
        if self.cursor_x > 0 {
            if !ctrl {
                self.lines[self.cursor_y].remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            } else {
                let start = self.lines[self.cursor_y].rfind(' ').unwrap_or(0);
                self.lines[self.cursor_y].drain(start..self.cursor_x);
                self.cursor_x = start;
            }

            execute!(
                io::stdout(),
                MoveTo(self.cursor_x as u16, self.cursor_y as u16),
                Clear(ClearType::UntilNewLine),
                Print(&self.lines[self.cursor_y][self.cursor_x..]),
                MoveTo(self.cursor_x as u16, self.cursor_y as u16),
            )?;
        } else if self.cursor_x == 0 && self.cursor_y > 0 {
            let current_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&current_line);

            queue!(
                io::stdout(),
                MoveTo(self.cursor_x as u16, self.cursor_y as u16),
                Print(&current_line),
            )?;

            for y in (self.cursor_y + 1)..self.lines.len() {
                queue!(
                    io::stdout(),
                    MoveTo(0, y as u16),
                    Clear(ClearType::CurrentLine),
                    Print(&self.lines[y]),
                )?;
            }
            execute!(
                io::stdout(),
                MoveTo(0, self.lines.len() as u16),
                Clear(ClearType::CurrentLine),
                MoveTo(self.cursor_x as u16, self.cursor_y as u16)
            )?;
        }

        Ok(())
    }

    pub(crate) fn handle_enter(&mut self) -> io::Result<()> {
        let right = self.lines[self.cursor_y].split_off(self.cursor_x);
        queue!(io::stdout(), Clear(ClearType::UntilNewLine))?;

        self.cursor_y += 1;
        self.cursor_x = 0;
        self.lines.insert(self.cursor_y, right);

        for y in self.cursor_y..self.lines.len() {
            queue!(
                io::stdout(),
                MoveTo(0, y as u16),
                Clear(ClearType::CurrentLine),
                Print(&self.lines[y]),
            )?;
        }
        execute!(io::stdout(), MoveTo(0, self.cursor_y as u16))?;

        Ok(())
    }

    pub(crate) fn move_left(&mut self) -> io::Result<()> {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_x == 0 && self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
        }

        execute!(
            io::stdout(),
            MoveTo(self.cursor_x as u16, self.cursor_y as u16)
        )?;

        Ok(())
    }

    pub(crate) fn move_right(&mut self) -> io::Result<()> {
        let line_length = self.lines[self.cursor_y].len();
        if self.cursor_x < line_length {
            self.cursor_x += 1;
        } else if self.cursor_x == line_length && self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }

        execute!(
            io::stdout(),
            MoveTo(self.cursor_x as u16, self.cursor_y as u16)
        )?;

        Ok(())
    }

    pub(crate) fn move_up(&mut self) -> io::Result<()> {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            let line_length = self.lines[self.cursor_y].len();
            self.cursor_x = min(self.cursor_x, line_length);
        }

        execute!(
            io::stdout(),
            MoveTo(self.cursor_x as u16, self.cursor_y as u16)
        )?;

        Ok(())
    }

    pub(crate) fn move_down(&mut self) -> io::Result<()> {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            let line_length = self.lines[self.cursor_y].len();
            self.cursor_x = min(self.cursor_x, line_length);
        }

        execute!(
            io::stdout(),
            MoveTo(self.cursor_x as u16, self.cursor_y as u16)
        )?;

        Ok(())
    }
}
