use crate::cursor::CursorPosition;
use crate::cursor::Cursor;

/// handles editor content
#[derive(Debug)]
pub struct Editor {
    pub editor_content: Vec<String>,
    pub visual_cursor_x: i16,
    pub cursor: Cursor, //to save position in editor, when toggling area
    pub text_selection_start: Option<CursorPosition>,
    pub text_selection_end: Option<CursorPosition>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            editor_content: vec![],
            visual_cursor_x: 0,
            text_selection_start: None,
            text_selection_end: None,
            cursor: Cursor::new(),
        }
    }



}