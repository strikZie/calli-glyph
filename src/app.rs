use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::{Instant, Duration};
use color_eyre::Result;
use ratatui::{

    DefaultTerminal
};



use crate::ui::{ui};
use crate::input::{handle_input};
use crate::config::editor_settings;

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    pub(crate) active_area: ActiveArea,
    pub(crate) editor_content: Vec<String>,
    pub(crate) command_input: String,
    pub(crate) file_path: Option<String>,
    pub(crate) cursor_x: i16,
    pub(crate) cursor_y: i16,    
    pub(crate) visual_cursor_x: i16,
    pub(crate) editor_cursor_x: i16, //to save position in editor, when toggling area
    pub(crate) editor_cursor_y: i16, //to save position in editor, when toggling are
    pub(crate) cursor_visible: bool,
    last_tick: Instant,
    pub(crate) scroll_offset: i16,
    pub(crate) terminal_height: i16,
    pub(crate) text_selection_start: Option<CursorPosition>,
    pub(crate) text_selection_end: Option<CursorPosition>,
    pub(crate) copied_text: Vec<String>,
}

#[derive(Debug,Copy,Clone)]
pub struct CursorPosition {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Default for CursorPosition {
    fn default() -> CursorPosition {
        CursorPosition { x: 0, y: 0 }
    }
}

#[derive(PartialEq, Debug, Default)]
pub(crate) enum ActiveArea {
    #[default]
    Editor,
    CommandLine,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: Default::default(),
            active_area: Default::default(),
            editor_content: vec!(String::new()),
            command_input: String::new(),
            file_path: None,
            cursor_x: 0,
            cursor_y: 0,
            visual_cursor_x: 0,
            editor_cursor_x: 0,
            editor_cursor_y: 0,
            last_tick: Instant::now(),
            cursor_visible: true,
            scroll_offset: 0,
            terminal_height: 0,
            text_selection_start: Default::default(),
            text_selection_end: Default::default(),
            copied_text: vec![],
        }
    }
}


impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal, file_path: Option<String>) -> Result<()> {
        //SETUP

        self.running = true;
        self.active_area = ActiveArea::Editor;
        self.file_path = file_path;

        // Read file contents if a file path is provided
        self.editor_content = if let Some(ref path) = self.file_path {
            match File::open(path) {
                Ok(f) => {
                    let mut buff_read_file = BufReader::new(f);
                    let mut contents = String::new();
                    match buff_read_file.read_to_string(&mut contents) {
                        Ok(_size) => contents.lines().map(String::from).collect(),
                        Err(err) => { //if file not found create new
                            self.running = false;
                            panic!(
                                "Failed to create file '{}': {}",
                                path, err
                            );
                        }
                    }
                },
                Err(_err) => {
                    match File::create(path) { //create file, if ok then return else quit and panic
                        Ok(_) => {
                            vec!(String::new()) // Return an empty string as the content
                        }
                        Err(create_err) => {
                            self.running = false;
                            panic!(
                                "Failed to create file '{}': {}",
                                path, create_err
                            );
                        }
                    }
                }
            }
        } else {
            vec!(String::new()) // Start with an empty editor if no file is provided
        };

        //LOGIC

        // Handle cursor blinking (toggle cursor visibility every 500ms)
        if self.last_tick.elapsed() >= Duration::from_millis(500) {
            self.cursor_visible = !self.cursor_visible;
            self.last_tick = Instant::now();
        }

        while self.running {
            terminal.draw(|frame| ui(frame, &mut self))?;
            handle_input(&mut self)?;
        }
        Ok(())
    }

    //TEXT OPERATIONS

    fn is_text_selected(&self) -> bool {
        if self.text_selection_start.is_some() && self.text_selection_end.is_some() {
            true
        } else {
            false
        }
    }


        //IN EDITOR
    ///wrapper function to either call write char with selected text or function write char,
    /// where text isn't selected
    pub(crate) fn write_all_char_in_editor(&mut self, c: char){
        if self.is_text_selected() {
            self.write_char_in_editor_text_is_selected(c)
        } else {
            self.write_char_in_editor(c)
        }
    }

    ///replaces all selected text with char to y position line, with x position
    fn write_char_in_editor_text_is_selected(&mut self, c: char){
        let start =self.text_selection_start.clone().unwrap();
        let end =self.text_selection_end.clone().unwrap();
        let lines = &mut self.editor_content[start.y..=end.y];
        let lines_length = lines.len().clone();
        if lines_length > 1 {
            for (y,line) in lines.iter_mut().enumerate() {
                let mut line_chars_vec:Vec<char> = line.chars().collect();

                //last line selected
                if y == lines_length -1 {
                    line_chars_vec.drain(0..end.x);
                } else {
                    line_chars_vec.drain(start.x..line.chars().count());
                }

                //start line selected
                if y == 0 {
                    line_chars_vec.insert(start.x,c);
                }

                *line = line_chars_vec.into_iter().collect();
            }
        } else {
            let line = &mut self.editor_content[start.y];
            let mut line_chars_vec:Vec<char> = line.chars().collect();
            line_chars_vec.drain(start.x..end.x);
            line_chars_vec.insert(start.x,c);
            *line = line_chars_vec.into_iter().collect();
        }
        self.cursor_x = self.text_selection_start.unwrap().x as i16;
        self.cursor_y = self.text_selection_start.unwrap().y as i16;
        self.text_selection_start = None;
        self.text_selection_end = None;
        self.move_cursor_in_editor(1, 0);
    }

    ///writes char to y position line, with x position
    pub(crate) fn write_char_in_editor(&mut self, c: char) {
        //creating lines until y position of cursor
        while self.editor_content.len() <= self.cursor_y as usize {
            self.editor_content.push(String::new());
        }

        let line = &mut self.editor_content[self.cursor_y as usize];

        let char_count = line.chars().count();
        //position cursor to line end in chars count
        if char_count < self.cursor_x as usize {
            self.cursor_x = char_count as i16;
        }

        let mut line_chars_vec:Vec<char> = line.chars().collect();

        line_chars_vec.insert(self.cursor_x as usize, c);

        *line = line_chars_vec.into_iter().collect();

        self.move_cursor_in_editor(1, 0);
    }

    ///wrapper function to either call backspace in editor with selected text or function backspace_in_editor,
    /// where text isn't selected
    pub(crate) fn backspace_all_in_editor(&mut self){
        if self.is_text_selected() {
            self.backspace_in_editor_text_is_selected();
        } else {
            self.backspace_in_editor();
        }
    }

    ///handles backspace in editor, removes char at y line x position and sets new cursor position
    pub(crate) fn backspace_in_editor_text_is_selected(&mut self) {
        let start =self.text_selection_start.clone().unwrap();
        let end =self.text_selection_end.clone().unwrap();
        let lines = &mut self.editor_content[start.y..=end.y];
        let lines_length = lines.len().clone();
        if lines_length > 1 {
            for (y,line) in lines.iter_mut().enumerate() {
                let mut line_chars_vec:Vec<char> = line.chars().collect();
                //last line selected
                if y == lines_length -1 {
                    line_chars_vec.drain(0..end.x);
                } else {
                    line_chars_vec.drain(start.x..line.chars().count());
                }

                *line = line_chars_vec.into_iter().collect();
            }
        } else {
            let line = &mut self.editor_content[start.y];
            let mut line_chars_vec:Vec<char> = line.chars().collect();
            line_chars_vec.drain(start.x..end.x);
            *line = line_chars_vec.into_iter().collect();
        }
        self.cursor_x = self.text_selection_start.unwrap().x as i16;
        self.cursor_y = self.text_selection_start.unwrap().y as i16;
        self.text_selection_start = None;
        self.text_selection_end = None;
        //replace visual cursor
        self.visual_cursor_x = self.calculate_visual_x() as i16;
    }

    ///handles backspace in editor, removes char at y line x position and sets new cursor position
    pub(crate) fn backspace_in_editor(&mut self) {
        let line_char_count = self.editor_content[self.cursor_y as usize].chars().count() as i16;
        if self.cursor_x > 0 && self.cursor_x <= line_char_count {
            let line = &mut self.editor_content[self.cursor_y as usize];
            let mut line_chars_vec:Vec<char> = line.chars().collect();

            line_chars_vec.remove(self.cursor_x as usize -1);

            *line = line_chars_vec.into_iter().collect();
            //line.remove(self.cursor_x as usize -1);
            self.move_cursor_in_editor(-1, 0);
        } else if self.cursor_y > 0 {
            let line = &mut self.editor_content.remove(self.cursor_y as usize);
            let new_x_value = self.editor_content[(self.cursor_y -1) as usize].chars().count() as i16;
            self.cursor_y -= 1;
            self.cursor_x = new_x_value;
            self.editor_content[self.cursor_y as usize].push_str(&line);
        }
    }


    ///wrapper function to either call backspace in editor with selected text or function backspace_in_editor,
    /// where text isn't selected
    pub(crate) fn delete_all_in_editor(&mut self){
        if self.is_text_selected() {
            self.delete_in_editor_text_is_selected();
        } else {
            self.delete_in_editor();
        }
    }

    ///handles delete in editor, removes char at y line x position and sets new cursor position
    pub(crate) fn delete_in_editor_text_is_selected(&mut self) {
        let start =self.text_selection_start.clone().unwrap();
        let end =self.text_selection_end.clone().unwrap();
        let lines = &mut self.editor_content[start.y..=end.y];
        let lines_length = lines.len().clone();
        if lines_length > 1 {
            for (y,line) in lines.iter_mut().enumerate() {
                let mut line_chars_vec:Vec<char> = line.chars().collect();

                //last line selected
                if y == lines_length -1 {
                    //line_chars_vec.drain(0..end.x);
                    line_chars_vec[0..end.x].fill(' ');
                } else {
                    //line_chars_vec[start.x..line.chars().count()].fill(' ');
                    line_chars_vec.drain(start.x..line.chars().count());
                }

                *line = line_chars_vec.into_iter().collect();
            }
        } else {
            let line = &mut self.editor_content[start.y];
            let mut line_chars_vec:Vec<char> = line.chars().collect();
            line_chars_vec[start.x..end.x].fill(' ');
            //line_chars_vec.drain(start.x..end.x);
            *line = line_chars_vec.into_iter().collect();
        }
        self.cursor_x = self.text_selection_end.unwrap().x as i16;
        self.cursor_y = self.text_selection_end.unwrap().y as i16;
        self.text_selection_start = None;
        self.text_selection_end = None;
        //replace visual cursor
        self.visual_cursor_x = self.calculate_visual_x() as i16;
    }

    ///handles DELETE action, of deleting char in editor at x +1 position
    pub(crate) fn delete_in_editor(&mut self) {
        let current_line_len = self.editor_content[self.cursor_y as usize].chars().count() as i16;

        if current_line_len == 0 { return; }
        //if at line end, move line below up,  else if current line length is bigger than current cursor x pos, remove char
        if self.cursor_x >= current_line_len -1 && self.editor_content.len() > (self.cursor_y +1) as usize {
            let line = &mut self.editor_content.remove((self.cursor_y +1) as usize);
            self.editor_content[self.cursor_y as usize].push_str(&line);
        } else if current_line_len > (self.cursor_x+1) {
            let line = &mut self.editor_content[self.cursor_y as usize];
            let mut line_chars_vec:Vec<char> = line.chars().collect();

            line_chars_vec.remove(self.cursor_x as usize +1);

            *line = line_chars_vec.into_iter().collect();
            //line.remove((self.cursor_x+1) as usize);
        }
    }

    ///handles TAB action in editor, by writing \t to editor content.
    pub(crate) fn tab_in_editor(&mut self) {

        let line = &mut self.editor_content[self.cursor_y as usize];

        let mut line_chars_vec:Vec<char> = line.chars().collect();

        line_chars_vec.insert(self.cursor_x as usize, '\t');

        *line = line_chars_vec.into_iter().collect();

        self.move_cursor_in_editor(1,0)
    }

    ///handles enter new line, with possible move of text
    pub(crate) fn enter_in_editor(&mut self) {
        let line = &mut self.editor_content[self.cursor_y as usize];
        //if at end of line len, then just move cursor and make new line, else move text too
        if self.cursor_x >= line.chars().count() as i16 {
            self.editor_content.insert(self.cursor_y as usize +1,String::new());
            self.move_cursor_in_editor(0,1);
        } else {
            //split current line and remove split part
            let mut line_chars_vec:Vec<char> = line.chars().collect();
            let line_end = line_chars_vec.split_off(self.cursor_x as usize);
            *line = line_chars_vec.into_iter().collect();

            //move down and insert split line to next line
            self.move_cursor_in_editor(0,1);
            self.editor_content.insert(self.cursor_y as usize,String::new());
            self.editor_content[self.cursor_y as usize] = line_end.into_iter().collect();
            self.cursor_x = 0;

        }
    }

        //IN COMMANDLINE

    /// writes char to the commandline content at x position, and moves cursor
    pub(crate) fn write_char_to_command_line(&mut self, c: char) {
        let line = &mut self.command_input;
        if line.len() < self.cursor_x as usize {
            self.cursor_x = line.len() as i16;
        }
        line.insert(self.cursor_x as usize, c);
        self.move_cursor_in_command_line(1);
    }

    pub(crate) fn backspace_on_command_line(&mut self) {
        let line = &mut self.command_input;
        if self.cursor_x > 0 && self.cursor_x <= line.len() as i16 {
            line.remove(self.cursor_x as usize -1);
            self.move_cursor_in_command_line(-1);
        }
    }


    //CURSOR
        //IN EDITOR
    ///calculates the visual position of the cursor
    fn calculate_visual_x(&mut self) -> usize {
        let line = &self.editor_content[self.cursor_y as usize];
        let cursor_x = self.cursor_x as usize;
        let tab_width = editor_settings::TAB_WIDTH as usize;
        let mut visual_x = 0;
        for (i, c) in line.chars().enumerate() {
            if i == cursor_x {
                break;
            }

            if c == '\t' {
                visual_x += tab_width - (visual_x % tab_width);
            } else {
                visual_x += 1;
            }

        }


        visual_x
    }

    ///wrapper function to either call move text selection cursor in editor or call to move cursor in editor,
    pub(crate) fn move_all_cursor_editor(&mut self, x: i16, y: i16, shift_held:bool) {

        if shift_held {
            self.move_selection_cursor(x,y);
        }else {
            self.move_cursor_in_editor(x,y);
            self.text_selection_start = None;
            self.text_selection_end = None;
        }

    }


    ///moves logical cursor by x and y, under conditions. and recalculates the visual cursor position
    pub(crate) fn move_cursor_in_editor(&mut self, x: i16, y: i16) {
        if self.cursor_y == 0 && y == -1 {
            return;
        }
        //if wanting to go beyond current length of editor
        while self.editor_content.len() <= (self.cursor_y + y) as usize {
            self.editor_content.push(String::new());
        }

        let max_x_pos = self.editor_content[(self.cursor_y + y) as usize].chars().count() as i16;
        //let current_line = &self.editor_content[self.cursor_y as usize];

        // Moving Right →
        if x > 0 && self.cursor_x < max_x_pos {
            self.cursor_x += x;
        }else if  x == 1 && self.cursor_x >= self.editor_content[self.cursor_y as usize].chars().count() as i16
            && self.editor_content.len() > self.cursor_y as usize +1{ //else if end of line and more lines
            self.cursor_y += 1;
            self.cursor_x = 0;
            self.visual_cursor_x = self.calculate_visual_x() as i16;
            return;
        }

        // Moving Left ←
        if x < 0 && self.cursor_x > 0 {
            self.cursor_x += x;
        } else if self.cursor_x == 0 && x == -1 && self.cursor_y != 0 { //else if start of line and more lines
            self.cursor_y -= 1;
            self.cursor_x = self.editor_content[self.cursor_y as usize].chars().count() as i16;
            self.visual_cursor_x = self.calculate_visual_x() as i16;
            return;
        }


        let (top, bottom) = self.is_cursor_top_or_bottom();
        //to offset scroll
        if (y == 1 && bottom) || (y == -1 && top) {
            self.scroll_offset = (self.scroll_offset + y).clamp(0, i16::MAX);
            return;
        }

        self.cursor_x = self.cursor_x.clamp(0, max_x_pos);
        self.cursor_y = (self.cursor_y + y).clamp(0, i16::MAX);
        self.visual_cursor_x = self.calculate_visual_x() as i16;
    }


    ///checks if cursor is at top or bottom of the screen
    fn is_cursor_top_or_bottom(&self) -> (bool,bool) {
        let top = self.cursor_y == self.scroll_offset;
        let bottom =  self.cursor_y == self.scroll_offset + (self.terminal_height -2);
        (top,bottom)
    }

    ///moves selection cursor
    pub(crate) fn move_selection_cursor(&mut self, x: i16, y: i16) {
        let old_x = self.cursor_x.clone();
        let old_y = self.cursor_y.clone();
        self.move_cursor_in_editor(x,y);
        let new_x = self.cursor_x.clone();
        let new_y = self.cursor_y.clone();

        let mut start_cp = CursorPosition::default();
        let mut end_cp = CursorPosition::default();
        if x > 0 || y > 0 {
            start_cp = CursorPosition{ x: old_x as usize, y: old_y as usize };
            end_cp = CursorPosition{ x: new_x as usize, y: new_y as usize };

            if self.text_selection_start.is_none() {
                self.text_selection_start = Option::from(start_cp);
            }
            self.text_selection_end = Option::from(end_cp);
        }

        if x < 0 || y < 0 {
            start_cp = CursorPosition{ x: new_x as usize, y: new_y as usize };
            end_cp = CursorPosition{ x: old_x as usize, y: old_y as usize };
            self.text_selection_start = Option::from(start_cp);
            if self.text_selection_end.is_none() {
                self.text_selection_end = Option::from(end_cp);
            }
        }

    }

        //IN COMMAND LINE
    ///moves cursor by x and y amounts in commandline
    pub(crate) fn move_cursor_in_command_line(&mut self, x: i16) {
        let max_x_pos:i16 = self.command_input.len() as i16;
        self.cursor_x = (self.cursor_x + x).clamp(0, max_x_pos);

    }


    //SCROLL
    ///moves the scroll offset
    pub(crate) fn move_scroll_offset(&mut self, offset: i16) {
        let (top, bottom) = self.is_cursor_top_or_bottom();

        //if on way down and at bottom, move scroll
        if (offset == 1 && bottom) || (offset == -1 && top) {
            self.scroll_offset = (self.scroll_offset + offset).clamp(0, i16::MAX);
            return;
        }

        self.move_cursor_in_editor(0, offset);
    }





    //PANEL HANDLING
    ///toggles the active area of the app, between editor and command line
    pub(crate) fn toggle_active_area(&mut self) {
        match self.active_area {
            ActiveArea::Editor =>  {
                self.editor_cursor_x = self.cursor_x;
                self.editor_cursor_y = self.cursor_y;
                self.active_area = ActiveArea::CommandLine;
                self.cursor_x = 0;
                self.cursor_y = 0;
            },
            ActiveArea::CommandLine => {
                self.active_area = ActiveArea::Editor;
                self.cursor_x = self.editor_cursor_x;
                self.cursor_y = self.editor_cursor_y;
            },

        }
    }


    //Basic Commands

    /// Set running == false, to quit the application.
    pub(crate) fn quit(&mut self) {
        self.running = false;
    }

    ///saves contents to file
    pub(crate) fn save(&self) -> Result<()> {
        let path;
        let has_changes:bool;

        let new_content = self.editor_content.join("\n");
        if self.file_path.is_some() {
            path = self.file_path.clone().unwrap();
            has_changes = self.file_has_changes(new_content.clone(),path.clone())?;
        }else {
            path = "untitled".to_string();
            has_changes = new_content.len() > 0;
        }

        if has_changes {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
            let mut buff_write_file = BufWriter::new(file);
            buff_write_file.write_all(new_content.as_bytes())?;
            buff_write_file.flush()?;
            Ok(())
        }else {
            Ok(())
        }
    }


    ///saves file and exits window
    pub(crate) fn save_and_exit(&mut self) -> Result<()> {
        match self.save() {
            Ok(_) => {
                self.quit();
                Ok(())
            },
            Err(e) => Err(e),
        }

    }

    ///copies text within bound of text selected to copied_text
    pub(crate) fn copy_selected_text(&mut self) -> Result<()> {
        if let (Some(start), Some(end)) = (self.text_selection_start.clone(), self.text_selection_end.clone()) {
            let mut selected_text: Vec<String> = Vec::new();
            let lines = &self.editor_content[start.y..=end.y];

            if lines.len() > 1 {
                for (y, line) in lines.iter().enumerate() {
                    let mut line_chars: Vec<char> = line.chars().collect();
                    let extracted_text: String;
                    
                    //if first line, else if last line, else 
                    if y == 0 {
                        extracted_text = line_chars.drain(start.x..).collect();
                    } else if y == lines.len() - 1 {
                        extracted_text = line_chars.drain(..end.x).collect();
                    } else {
                        extracted_text = line_chars.into_iter().collect();
                    }

                    selected_text.push(extracted_text);
                }
            } else {
                let mut line_chars: Vec<char> = self.editor_content[start.y].chars().collect();
                let extracted_text: String = line_chars.drain(start.x..end.x).collect();
                selected_text.push(extracted_text);
            }

            self.copied_text = selected_text.clone();
            Ok(())
        } else {
            Ok(())
        }
    }

    ///pastes text from copied text to editor content
    pub(crate) fn paste_selected_text(&mut self) -> Result<()> {
        //if no text in copied text
        if self.copied_text.is_empty() {
            return Ok(());
        }

        let insert_y = self.cursor_y as usize;
        let insert_x = self.cursor_x as usize;


        while self.editor_content.len() < insert_y + self.copied_text.len() -1 {
            self.editor_content.push(String::new());
        }

        let current_line = &self.editor_content[insert_y];

        // Convert the line into a Vec<char> to handle multi-byte characters correctly
        let chars: Vec<char> = current_line.chars().collect();
        let (before_cursor, after_cursor) = chars.split_at(insert_x.min(chars.len()));

        if self.copied_text.len() == 1 {
            // Single-line paste: correctly insert into character-safe split
            let new_line = format!(
                "{}{}{}",
                before_cursor.iter().collect::<String>(),
                self.copied_text[0],
                after_cursor.iter().collect::<String>()
            );
            self.editor_content[insert_y] = new_line;
        } else {
            // Multi-line paste
            let mut new_lines = Vec::new();

            // First line: insert copied text at cursor position
            new_lines.push(format!(
                "{}{}",
                before_cursor.iter().collect::<String>(),
                self.copied_text[0]
            ));

            // Middle lines: insert as separate lines
            for line in &self.copied_text[1..self.copied_text.len() - 1] {
                new_lines.push(line.clone());
            }

            // Last copied line + remainder of the original line
            let last_copied_line = &self.copied_text[self.copied_text.len() - 1];
            new_lines.push(format!(
                "{}{}",
                last_copied_line,
                after_cursor.iter().collect::<String>()
            ));

            // Replace the current line and insert new lines
            self.editor_content.splice(insert_y..=insert_y, new_lines);
        }

        // Clear copied text after pasting
        //self.copied_text.clear();
        Ok(())

    }

    //HELPER FUNCTIONS FOR BASIC COMMANDS
    ///checks if file has changes and returns boolean
    pub(crate) fn file_has_changes(&self,editor_content:String,file_path:String) -> Result<bool> {

        let file = File::open(file_path)?;
        let mut buff_read_file = BufReader::new(file);
        let mut read_file_contents = String::new();

        buff_read_file.read_to_string(&mut read_file_contents).expect("TODO: panic message");
        //if has changes, return true else return false
        if !read_file_contents.eq(&editor_content) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

}
