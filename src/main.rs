mod terminal;
mod editor;
mod error;

use std::io::Read;
use std::os::fd::AsRawFd;
use crate::editor::Editor;
use crate::terminal::die;


fn run(mut editor: Editor) {
    loop {
        editor.refresh_screen();
        editor.handle_keypress();
    }

}

fn main() {

    let try_editor = Editor::new();
    match try_editor {
        Ok(editor) => run(editor),
        Err(e) => die(&e.to_string())
    }

}
