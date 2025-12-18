mod editor;
mod key_handlers;
mod raw_mode_guard;

use editor::Editor;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let filename = args.nth(1);

    let mut editor = Editor::new();
    if let Some(filename) = filename {
        editor.load_file(&filename)?;
    }
    editor.run()?;
    
    Ok(())
}
