use std::io::Write;
use crate::editor::Direction::{Down, Left, Right, Up};
use crate::editor::Key::{Arrow, Delete, End, Home, PageDown, PageUp, Simple};
use crate::terminal::{ctrl, Terminal};
use crate::error::Result;

const APP_NAME: &str = "kilo-rs";
const APP_VERSION: &str = "0.0.1";

const ESC: u8 = 0x1b;

enum Direction {
    Left,
    Right,
    Up,
    Down
}
enum Key {
    Arrow(Direction),
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Simple(u8)
}

pub(crate) struct Editor {
    pub terminal: Terminal,
    rows: usize,
    cols: usize,
    c_row: usize,
    c_col: usize
}

impl Editor {

    pub(crate) fn new() -> Result<Editor> {
        let terminal = Terminal::new()?;
        terminal.enable_raw_mode();
        let (rows, cols) = terminal.screen_size()?;
        Ok(Editor { terminal, rows, cols, c_row: 0, c_col: 0 })
    }

    /// Called where ESC key is read from stdin. Checks to see if it is the start of a sequence and returns the
    /// appropriate key if so. Otherwise, returns ESC.
    fn handle_esc_key(&mut self) -> Result<Key> {
        let b1 = self.terminal.read_key()?;
        let b2 = self.terminal.read_key()?;
        // Note: Could (partially) flatten this by using `match (b1, b2)`
        if b1 == b'[' {
            match b2 {
                b'A' => Ok(Arrow(Up)),
                b'B' => Ok(Arrow(Down)),
                b'C' => Ok(Arrow(Right)),
                b'D' => Ok(Arrow(Left)),
                b'H' => Ok(Home),
                b'F' => Ok(End),
                b'0'..=b'9' => {
                    if self.terminal.read_byte()? == b'~' {
                        match b2 {
                            b'1' => Ok(Home),
                            b'3' => Ok(Delete),
                            b'4' => Ok(End),
                            b'5' => Ok(PageUp),
                            b'6' => Ok(PageDown),
                            b'7' => Ok(Home),
                            b'8' => Ok(End),
                            _ => Ok(Simple(ESC))
                        }
                    } else {
                        Ok(Simple(ESC))
                    }
                },
                _ => Ok(Simple(ESC))
            }
        } else if b1 == b'0' {
            match b2 {
                b'H' => Ok(Home),
                b'F' => Ok(End),
                _ => Ok(Simple(ESC))
            }
        } else {
            Ok(Simple(ESC))
        }
    }

    fn get_key(&mut self) -> Result<Key> {
        let mut b = self.terminal.read_key()?;
        if b == ESC {
            return Ok(self.handle_esc_key().unwrap_or(Simple(ESC)));
        }
        Ok(Simple(b))
    }
    pub(crate) fn handle_keypress(&mut self) -> Result<()> {
        let k = self.get_key()?;
        match k {
            Arrow(d) => self.move_cursor(d)?,
            PageUp => {
                for _ in 0..self.rows {
                    self.move_cursor(Up)?
                }
            },
            PageDown => {
                for _ in 0..self.rows {
                    self.move_cursor(Down)?
                }
            }
            Home => self.c_col = 0,
            End => self.c_col = self.cols - 1,
            Simple(b) => {
                if b == ctrl(b'q') {
                    self.terminal.clean_exit()?
                }
            },
            _ => ()
        }
        Ok(())
    }

    fn move_cursor(&mut self, direction: Direction) -> Result<()> {
        match direction {
            Left => if self.c_col > 0 { self.c_col -= 1 },
            Right => if self.c_col < self.cols - 1 { self.c_col += 1 },
            Up => if self.c_row > 0 { self.c_row -= 1 },
            Down => if self.c_row < self.rows - 1 { self.c_row += 1 }
        }
        Ok(())
    }

    pub(crate) fn refresh_screen(&self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.reset_cursor()?;
        self.draw_rows()?;
        self.terminal.move_cursor_to(self.c_row + 1, self.c_col + 1)?;
        self.terminal.show_cursor()?;
        self.terminal.flush()
    }

    pub(crate) fn draw_rows(&self) -> Result<()> {
        for r in 0..self.rows {
            if r == self.rows / 3 {
                let mut welcome = format!("{APP_NAME} -- version {APP_VERSION}");
                if welcome.len() > self.cols {
                    welcome.truncate(self.cols)
                }
                let padding = (self.cols - welcome.len()) / 2;
                if padding > 0 {
                    self.terminal.write(b"~")?;
                    for _ in 1..padding {
                        self.terminal.write(b" ")?;
                    }
                }
                self.terminal.write_str(&welcome)?;
            } else {
                self.terminal.write(b"~")?;
            }
            self.terminal.write(b"\x1b[K")?; // Erase line to right of cursor
            if r < self.rows - 1 {
                self.terminal.write(b"\r\n")?;
            }
        }
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.disable_raw_mode().expect("Could not disable raw mode in terminal.")
    }
}