use crate::cursor::Cursor;

#[derive(Debug)]
pub struct CommandLine {
    pub input: String,
    pub cursor: Cursor,
}

impl CommandLine {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: Cursor::new(),
        }
    }

}