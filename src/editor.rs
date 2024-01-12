use std::io::Write;
use crate::terminal::{ctrl_key, Terminal};
use crate::error::Result;

const APP_NAME: &str = "kilo-rs";
const APP_VERSION: &str = "0.0.1";

pub(crate) struct Editor {
    pub terminal: Terminal,
    rows: usize,
    cols: usize,
    buf: Vec<u8>
}

impl Editor {

    pub(crate) fn new() -> Result<Editor> {
        let terminal = Terminal::new()?;
        terminal.enable_raw_mode();
        let (rows, cols) = terminal.screen_size()?;
        let buf: Vec<u8> = vec!();
        Ok(Editor { terminal, rows, cols, buf })
    }

    pub(crate) fn process_keypress(&self) -> Result<()> {
        let b = self.terminal.read_key()?;
        if b == ctrl_key(b'q') {
            return self.terminal.clean_exit()
        }
        Ok(())
    }

    pub(crate) fn refresh_screen(&self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.reset_cursor()?;
        self.draw_rows()?;
        self.terminal.reset_cursor()?;
        self.terminal.show_cursor()
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
        self.terminal.flush()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.disable_raw_mode().expect("Could not disable raw mode in terminal.")
    }
}