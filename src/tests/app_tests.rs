#[cfg(test)]
mod app_tests {
    use crate::app::*;

    //init functions
    fn create_app() -> App {
        let mut app = App::new();
        app
    }
    #[test]
    fn test_toggle_to_command_line() {
        let mut app = create_app();
        app.active_area = ActiveArea::Editor;
        app.editor.cursor.x = 5;
        app.editor.cursor.y = 3;

        app.toggle_active_area();
        assert_eq!(app.active_area, ActiveArea::CommandLine);
        assert_eq!(app.command_line.cursor.x, 0);
        assert_eq!(app.command_line.cursor.y, 0);
        assert_eq!(app.editor.cursor.x, 5);
        assert_eq!(app.editor.cursor.y, 3);
    }

    #[test]
    fn test_toggle_to_editor() {
        let mut app = create_app();
        app.active_area = ActiveArea::CommandLine;
        app.editor.cursor.x = 5;
        app.editor.cursor.y = 3;

        app.toggle_active_area();
        assert_eq!(app.active_area, ActiveArea::Editor);
        assert_eq!(app.editor.cursor.x, 5);
        assert_eq!(app.editor.cursor.y, 3);
    }
}


#[cfg(test)]
mod app_editor_tests {
    use crate::app::*; // Access app.rs logic
    use crate::config::editor_settings;
    use crate::cursor::CursorPosition;

    //init functions
    fn create_app_with_editor_content(vec: Vec<String>) -> App {
        let mut app = App::new();
        app.editor.editor_content = vec;
        app

    }

    //WRITING CHARS IN EDITOR

    #[test]
    fn test_write_char_in_editor() {
        let mut app = App::new();
        app.write_char_in_editor('a');
        assert_eq!(app.editor.editor_content[0], "a");
        assert_eq!(app.editor.cursor.x, 1);
    }

    #[test]
    fn test_write_char_in_editor_normal_characters() {
        let mut app = App::new();
        app.write_char_in_editor('a');
        app.write_char_in_editor('b');
        app.write_char_in_editor('c');
        app.write_char_in_editor('d');
        assert_eq!(app.editor.editor_content[0], "abcd");
        assert_eq!(app.editor.cursor.x, 4);
    }

    #[test]
    fn test_write_char_in_editor_special_characters() {
        let mut app = App::new();
        app.write_char_in_editor('ᚠ');
        app.write_char_in_editor('Ω');
        app.write_char_in_editor('₿');
        app.write_char_in_editor('😎');
        assert_eq!(app.editor.editor_content[0], "ᚠΩ₿😎");
        assert_eq!(app.editor.cursor.x, 4);
    }

    #[test]
    fn test_write_char_in_editor_at_line_10() {
        let mut app = App::new();
        app.editor.cursor.y = 10;
        app.write_char_in_editor('a');
        assert_eq!(app.editor.editor_content[10], "a");
        assert_eq!(app.editor.cursor.x, 1);
    }

    #[test]
    fn test_write_char_in_editor_at_100_x() {
        let mut app = App::new();
        app.editor.cursor.x = 100;
        app.write_char_in_editor('a');
        assert_eq!(app.editor.editor_content[0], "a");
        assert_eq!(app.editor.cursor.x, 1);
    }

    //Write char to editor with selected text
    #[test]
    fn test_write_char_in_editor_with_selected_text() {
        let mut app = create_app_with_editor_content(vec!["Hello Denmark".to_string()]);
        app.editor.text_selection_start = Option::Some(CursorPosition{ x: 6, y: 0 });
        app.editor.text_selection_end = Option::Some(CursorPosition{ x: 13, y: 0 });
        app.editor.cursor.x = 6;
        app.write_all_char_in_editor('W');
        assert_eq!(app.editor.editor_content[0], "Hello W");
        assert_eq!(app.editor.cursor.x, 7);
    }

    #[test]
    fn test_write_char_in_editor_with_selected_text_multiple_lines() {
        let mut app = create_app_with_editor_content(vec!["test".to_string(),"Hello Denmark".to_string(), "Hello Sudetenland".to_string()]);
        app.editor.text_selection_start = Option::Some(CursorPosition{ x: 6, y: 1 });
        app.editor.text_selection_end = Option::Some(CursorPosition{ x: 13, y: 2 });
        app.editor.cursor.x = 6;
        app.write_all_char_in_editor('W');
        assert_eq!(app.editor.editor_content[0], "test");
        assert_eq!(app.editor.editor_content[1], "Hello W");
        assert_eq!(app.editor.editor_content[2], "land");
        assert_eq!(app.editor.cursor.x, 7);
    }

    #[test]
    fn test_write_char_in_editor_with_selected_text_special_characters() {
        let mut app = create_app_with_editor_content(vec!["ᚠΩ₿😎".to_string()]);
        app.editor.text_selection_start = Option::Some(CursorPosition{ x: 1, y: 0 });
        app.editor.text_selection_end = Option::Some(CursorPosition{ x: 2, y: 0 });
        app.editor.cursor.x = 1;

        app.write_all_char_in_editor('a');
        assert_eq!(app.editor.editor_content[0], "ᚠa₿😎");
        assert_eq!(app.editor.cursor.x, 2);
    }

    //BACKSPACE IN EDITOR
    #[test]
    fn test_backspace_in_editor() {
        let mut app = create_app_with_editor_content(vec!['a'.to_string()]);
        app.editor.cursor.x = 1;
        app.backspace_in_editor();
        assert_eq!(app.editor.editor_content[0], "");
        assert_eq!(app.editor.cursor.x, 0);
    }

    #[test]
    fn test_backspace_in_editor_special_characters() {
        let mut app = create_app_with_editor_content(vec!["ᚠΩ₿😎".to_string()]);
        app.editor.cursor.x = 4;
        app.backspace_in_editor();
        assert_eq!(app.editor.editor_content[0], "ᚠΩ₿");
        assert_eq!(app.editor.cursor.x, 3);
    }

    #[test]
    fn test_backspace_in_editor_should_go_to_previous_line() {
        let mut app = create_app_with_editor_content(vec!["a".to_string(), "b".to_string()]);
        app.editor.cursor.y = 1;
        app.editor.cursor.x = 0;
        app.backspace_in_editor();
        assert_eq!(app.editor.editor_content[0], "ab");
        assert_eq!(app.editor.editor_content.len(), 1);
        assert_eq!(app.editor.cursor.x, 1);
        assert_eq!(app.editor.cursor.y, 0);

    }

        //TEXT IS SELECTED

    #[test]
    fn test_backspace_in_editor_text_is_selected() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["Hello Denmark".to_string()]);

        // Set a selection range (e.g., "Denmark")
        app.editor.text_selection_start = Some(CursorPosition{x: 6, y:0} ); // Start of "Denmark"
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:0});  // End of "Denmark"
        // Call the function to simulate a backspace with text selected
        app.backspace_in_editor_text_is_selected();

        // Assert that the selected text is removed
        assert_eq!(app.editor.editor_content, vec!["Hello "]);

        // Assert that the selection is cleared after the operation
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor is moved to the correct position
        assert_eq!(app.editor.cursor.x, 6);
        assert_eq!(app.editor.cursor.y, 0);

    }

    #[test]
    fn test_backspace_in_editor_text_is_selected_multiple_lines() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["test".to_string(),"Hello Denmark".to_string(), "Hello Sudetenland".to_string()]);

        // Set a selection range (e.g., "Denmark")
        app.editor.text_selection_start = Some(CursorPosition{x: 6, y:1} ); // Start of "Denmark"
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:2});  // End of "Denmark"
        // Call the function to simulate a backspace with text selected
        app.backspace_in_editor_text_is_selected();

        assert_eq!(app.editor.editor_content.len(), 3);

        // Assert that the selected text is removed
        assert_eq!(app.editor.editor_content[0], "test");
        assert_eq!(app.editor.editor_content[1], "Hello ");

        // Assert that the selection is cleared after the operation
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor is moved to the correct position
        assert_eq!(app.editor.cursor.x, 6);
        assert_eq!(app.editor.cursor.y, 1);

    }


    #[test]
    fn test_backspace_in_editor_text_is_selected_empty_text() {
        // Initialize the editor with empty content
        let mut app = create_app_with_editor_content(vec!["".to_string()]);

        // Set a selection range (even though the text is empty)
        app.editor.text_selection_start = Some(CursorPosition{x: 0, y:0} );
        app.editor.text_selection_end = Some(CursorPosition{x: 0, y:0});

        // Call the function to simulate a backspace with empty text
        app.backspace_in_editor_text_is_selected();

        // Assert that the text remains empty
        assert_eq!(app.editor.editor_content, vec!["".to_string()]);

        // Assert that the selection is cleared
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor position is 0
        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_backspace_in_editor_text_is_selected_full_text_selected() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["Hello Denmark".to_string()]);

        // Set a selection range for the entire text
        app.editor.text_selection_start = Some(CursorPosition{x: 0, y:0} );
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:0});

        // Call the function to simulate a backspace with the entire text selected
        app.backspace_in_editor_text_is_selected();

        // Assert that all text is removed
        assert_eq!(app.editor.editor_content, vec!["".to_string()]);

        // Assert that the selection is cleared
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor position is 0
        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 0);
    }

    //DELETE IN EDITOR
    #[test]
    fn test_delete_in_editor() {
        let mut app = create_app_with_editor_content(vec!["ab".to_string()]);
        app.editor.cursor.x = 0;
        app.delete_in_editor();
        assert_eq!(app.editor.editor_content[0], "a");
        assert_eq!(app.editor.cursor.x, 0);
    }

    #[test]
    fn test_delete_in_editor_special_characters() {
        let mut app = create_app_with_editor_content(vec!["ᚠΩ₿😎".to_string(),]);
        app.editor.cursor.x = 2;
        app.delete_in_editor();
        assert_eq!(app.editor.editor_content[0], "ᚠΩ₿");
        assert_eq!(app.editor.cursor.x, 2);
    }

    #[test]
    fn test_delete_in_editor_should_go_to_previous_line() {
        let mut app = create_app_with_editor_content(vec!["a".to_string(), "b".to_string()]);
        app.editor.cursor.x = 1;
        app.delete_in_editor();
        assert_eq!(app.editor.editor_content[0], "ab");
        assert_eq!(app.editor.editor_content.len(), 1);
        assert_eq!(app.editor.cursor.x, 1);
    }

    //TEXT IS SELECTED

    #[test]
    fn test_delete_in_editor_text_is_selected() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["Hello Denmark".to_string()]);

        // Set a selection range (e.g., "Denmark")
        app.editor.text_selection_start = Some(CursorPosition{x: 6, y:0} ); // Start of "Denmark"
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:0});  // End of "Denmark"
        // Call the function to simulate a backspace with text selected
        app.delete_in_editor_text_is_selected();

        // Assert that the selected text is removed
        assert_eq!(app.editor.editor_content[0], "Hello        ");
        assert_eq!(app.editor.editor_content[0].len(), 13);

        // Assert that the selection is cleared after the operation
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor is moved to the correct position
        assert_eq!(app.editor.cursor.x, 13);
        assert_eq!(app.editor.cursor.y, 0);

    }

    #[test]
    fn test_delete_in_editor_text_is_selected_multiple_lines() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["test".to_string(),"Hello Denmark".to_string(), "Hello Sudetenland".to_string()]);

        // Set a selection range (e.g., "Denmark")
        app.editor.text_selection_start = Some(CursorPosition{x: 6, y:1} ); // Start of "Denmark"
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:2});  // End of "Denmark"
        // Call the function to simulate a backspace with text selected
        app.delete_in_editor_text_is_selected();

        assert_eq!(app.editor.editor_content.len(), 3);

        // Assert that the selected text is removed
        assert_eq!(app.editor.editor_content[0], "test");
        assert_eq!(app.editor.editor_content[1], "Hello ");
        assert_eq!(app.editor.editor_content[2].len(), 17);

        // Assert that the selection is cleared after the operation
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor is moved to the correct position
        assert_eq!(app.editor.cursor.x, 13);
        assert_eq!(app.editor.cursor.y, 2);

    }


    #[test]
    fn test_delete_in_editor_text_is_selected_empty_text() {
        // Initialize the editor with empty content
        let mut app = create_app_with_editor_content(vec!["".to_string()]);

        // Set a selection range (even though the text is empty)
        app.editor.text_selection_start = Some(CursorPosition{x: 0, y:0} );
        app.editor.text_selection_end = Some(CursorPosition{x: 0, y:0});

        // Call the function to simulate a backspace with empty text
        app.delete_in_editor_text_is_selected();

        // Assert that the text remains empty
        assert_eq!(app.editor.editor_content, vec!["".to_string()]);

        // Assert that the selection is cleared
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor position is 0
        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_delete_in_editor_text_is_selected_full_text_selected() {
        // Initialize the editor with some content
        let mut app = create_app_with_editor_content(vec!["Hello Denmark".to_string()]);

        // Set a selection range for the entire text
        app.editor.text_selection_start = Some(CursorPosition{x: 0, y:0} );
        app.editor.text_selection_end = Some(CursorPosition{x: 13, y:0});

        // Call the function to simulate a backspace with the entire text selected
        app.delete_in_editor_text_is_selected();

        // Assert that all text is removed
        assert_eq!(app.editor.editor_content[0].len(), 13);

        // Assert that the selection is cleared
        assert!(app.editor.text_selection_start.is_none());
        assert!(app.editor.text_selection_end.is_none());

        // Assert that the cursor position is 0
        assert_eq!(app.editor.cursor.x, 13);
        assert_eq!(app.editor.cursor.y, 0);
    }


    //TAB in editor
    #[test]
    fn test_tab_in_editor_start_of_empty_line() {
        let mut app = create_app_with_editor_content(vec!["".to_string()]);
        app.tab_in_editor();

        assert_eq!(app.editor.cursor.y, 0); // Cursor should stay on line
        assert_eq!(app.editor.editor_content.len(), 1); // New line added
        assert_eq!(app.editor.visual_cursor_x, editor_settings::TAB_WIDTH as i16);
    }

    #[test]
    fn test_tab_in_editor_start_of_line() {
        let mut app = create_app_with_editor_content(vec!["HELLO WORLD".to_string()]);
        app.tab_in_editor();

        assert_eq!(app.editor.cursor.y, 0); // Cursor should stay on line
        assert_eq!(app.editor.editor_content.len(), 1); // New line added
        assert_eq!(app.editor.visual_cursor_x, editor_settings::TAB_WIDTH as i16);
    }

    #[test]
    fn test_tab_in_editor_mid_of_line_normal_characters() {
        let mut app = create_app_with_editor_content(vec!["1234".to_string()]);
        app.editor.cursor.x = 2;
        app.tab_in_editor();

        assert_eq!(app.editor.cursor.y, 0); // Cursor should stay on line
        assert_eq!(app.editor.editor_content.len(), 1); // New line added
        assert_eq!(app.editor.visual_cursor_x, 4);
        app.move_cursor_in_editor(10,0); //move to end
        assert_eq!(app.editor.editor_content[0].chars().count(), 5); //should contain special plus \t char
        assert_eq!(app.editor.visual_cursor_x, 6); //at end of line should be 6
    }

    #[test]
    fn test_tab_in_editor_mid_of_line_special_characters() {
        let mut app = create_app_with_editor_content(vec!["ᚠΩ₿😎".to_string()]);
        app.editor.cursor.x = 2;
        app.tab_in_editor();

        assert_eq!(app.editor.cursor.y, 0); // Cursor should stay on line
        assert_eq!(app.editor.editor_content.len(), 1); // New line added
        assert_eq!(app.editor.visual_cursor_x, 4);
        app.move_cursor_in_editor(10,0); //move to end
        assert_eq!(app.editor.editor_content[0].chars().count(), 5); //should contain special plus \t char
        assert_eq!(app.editor.visual_cursor_x, 6); //at end of line should be 6
    }


    //ENTER in editor

    #[test]
    fn test_enter_in_editor_at_end_of_line() {
        let mut app = create_app_with_editor_content(vec!["Hello World".to_string()]);
        app.editor.cursor.x = app.editor.editor_content[0].len() as i16; // Set cursor to end of line
        app.enter_in_editor();

        assert_eq!(app.editor.cursor.y, 1); // Cursor should move to the next line
        assert_eq!(app.editor.editor_content.len(), 2); // New line added
        assert_eq!(app.editor.editor_content[1], ""); // New line should be empty
    }

    #[test]
    fn test_enter_in_editor_mid_line() {
        let mut app = create_app_with_editor_content(vec!["Hello World".to_string()]);
        app.editor.cursor.x = 5; // Split the line at index 5
        app.enter_in_editor();

        assert_eq!(app.editor.cursor.y, 1); // Cursor should move to next line
        assert_eq!(app.editor.cursor.x, 0); // Cursor resets to start of new line
        assert_eq!(app.editor.editor_content[0], "Hello"); // Line before cursor is kept intact
        assert_eq!(app.editor.editor_content[1], " World"); // Line after cursor is moved to new line
    }


    //MOVE CURSOR in editor

    #[test]
    fn test_cursor_move_right_within_line() {
        let mut app = create_app_with_editor_content(vec!["Hello World".to_string()]);
        app.move_cursor_in_editor(1, 0);

        assert_eq!(app.editor.cursor.x, 1);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_cursor_move_left_at_start_should_stay() {
        let mut app = create_app_with_editor_content(vec!["Hello World".to_string()]);
        app.move_cursor_in_editor(-1, 0);

        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_cursor_move_right_within_empty_line_should_stay() {
        let mut app = create_app_with_editor_content(vec![]);
        app.move_cursor_in_editor(1, 0);

        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_cursor_move_right_at_end_of_first_line_should_move_down() {
        let mut app = create_app_with_editor_content(vec!["First".to_string(),"Second".to_string()]);
        app.editor.cursor.x = 5;
        app.move_cursor_in_editor(1, 0);

        assert_eq!(app.editor.cursor.x, 0);
        assert_eq!(app.editor.cursor.y, 1);
    }

    #[test]
    fn test_cursor_move_right_at_end_of_first_line_should_stay() {
        let mut app = create_app_with_editor_content(vec!["First".to_string()]);
        app.editor.cursor.x = 5;
        app.move_cursor_in_editor(1, 0);

        assert_eq!(app.editor.cursor.x, 5);
        assert_eq!(app.editor.cursor.y, 0);
    }

    #[test]
    fn test_cursor_move_down() {
        let mut app = create_app_with_editor_content(vec!["Second Line".to_string()]);
        app.move_cursor_in_editor(0, 1);

        assert_eq!(app.editor.cursor.x, 0); // Cursor stays at column 0
        assert_eq!(app.editor.cursor.y, 1); // Moves to the second line
    }


    //SELECTION CURSOR

    #[test]
    fn test_selection_cursor_move_up_should_stay() {
        let mut app = create_app_with_editor_content(vec![]);
        app.move_selection_cursor(0, -1);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    #[test]
    fn test_selection_cursor_move_down_go_down() {
        let mut app = create_app_with_editor_content(vec![]);
        app.move_selection_cursor(0, 1);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 1);
    }

    #[test]
    fn test_selection_cursor_move_left_should_stay() {
        let mut app = create_app_with_editor_content(vec![]);
        app.move_selection_cursor(-1, 0);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    #[test]
    fn test_selection_cursor_move_right_should_stay() {
        let mut app = create_app_with_editor_content(vec![]);
        app.move_selection_cursor(1, 0);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    #[test]
    fn test_selection_cursor_move_up_should_go_up() {
        let mut app = create_app_with_editor_content(vec!["First".to_string(),"Second".to_string()]);
        app.editor.cursor.y = 1;
        app.move_selection_cursor(0, -1);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 1);
    }

    #[test]
    fn test_selection_cursor_move_down_should_go_down() {
        let mut app = create_app_with_editor_content(vec!["First".to_string(),"Second".to_string()]);
        app.move_selection_cursor(0, 1);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 1);
    }

    #[test]
    fn test_selection_cursor_move_left_should_go_left() {
        let mut app = create_app_with_editor_content(vec!["First".to_string()]);
        app.editor.cursor.x = 1;
        app.move_selection_cursor(-1, 0);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 1);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    #[test]
    fn test_selection_cursor_move_right_should_go_right() {
        let mut app = create_app_with_editor_content(vec!["First".to_string()]);
        app.move_selection_cursor(1, 0);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 1);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    #[test]
    fn test_selection_cursor_move_right_thrice_should_go_right() {
        let mut app = create_app_with_editor_content(vec!["First".to_string()]);
        app.move_selection_cursor(1, 0);
        app.move_selection_cursor(1, 0);
        app.move_selection_cursor(1, 0);

        assert_eq!(app.editor.text_selection_start.unwrap().x, 0);
        assert_eq!(app.editor.text_selection_start.unwrap().y, 0);
        assert_eq!(app.editor.text_selection_end.unwrap().x, 3);
        assert_eq!(app.editor.text_selection_end.unwrap().y, 0);
    }

    //copy selected text
    #[test]
    fn test_copy_single_line_selection() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string()]);
        app.editor.text_selection_start = Some(CursorPosition { x: 7, y: 0 });
        app.editor.text_selection_end = Some(CursorPosition { x: 12, y: 0 });

        let result = app.copy_selected_text();

        assert!(result.is_ok());
        assert_eq!(app.clipboard.copied_text, vec!["world".to_string()]);
    }

    #[test]
    fn test_copy_multi_line_selection() {
        let mut app = create_app_with_editor_content(vec!["Hello,".to_string(), " world!".to_string(), " Rust".to_string()]);
        app.editor.text_selection_start = Some(CursorPosition { x: 4, y: 0 });
        app.editor.text_selection_end = Some(CursorPosition { x: 3, y: 2 });

        let result = app.copy_selected_text();

        assert!(result.is_ok());
        assert_eq!(app.clipboard.copied_text, vec!["o,", " world!", " Ru"].into_iter().map(String::from).collect::<Vec<String>>());
    }

    #[test]
    fn test_copy_no_selection() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string()]);
        app.editor.text_selection_start = None;
        app.editor.text_selection_end = None;

        let result = app.copy_selected_text();

        assert!(result.is_ok());
        assert!(app.clipboard.copied_text.is_empty());
    }

    //paste selected text
    #[test]
    fn test_paste_single_line() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["PASTED".to_string()]);
        app.editor.cursor.x = 8;
        app.editor.cursor.y = 0;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, wPASTEDorld!".to_string(),
                "This is a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_multiline() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["First".to_string(), "Second ".to_string()]);
        app.editor.cursor.x = 5;
        app.editor.cursor.y = 1;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, world!".to_string(),
                "This First".to_string(),
                "Second is a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_single_line_special_characters() {
        let mut app = create_app_with_editor_content(vec!["Hello, wᚠᚠᚠᚠorld!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["PASTED".to_string()]);
        app.editor.cursor.x = 10;
        app.editor.cursor.y = 0;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, wᚠᚠPASTEDᚠᚠorld!".to_string(),
                "This is a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_multiline_special_charaters() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This ᚠᚠᚠᚠis a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["First".to_string(), "Second ".to_string()]);
        app.editor.cursor.x = 7;
        app.editor.cursor.y = 1;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, world!".to_string(),
                "This ᚠᚠFirst".to_string(),
                "Second ᚠᚠis a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_at_start_of_line() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["NewStart".to_string()]);
        app.editor.cursor.x = 0;
        app.editor.cursor.y = 2;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
                "NewStartAnother line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_at_end_of_line() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec!["END".to_string()]);
        app.editor.cursor.x = 13;
        app.editor.cursor.y = 0;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, world!END".to_string(),
                "This is a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_with_empty_copied_text() {
        let mut app = create_app_with_editor_content(vec!["Hello, world!".to_string(),
                                                          "This is a test.".to_string(),
                                                          "Another line.".to_string(),]);
        app.clipboard.copy(&vec![]);
        app.editor.cursor.x = 5;
        app.editor.cursor.y = 1;

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
                "Another line.".to_string(),
            ]
        );
    }

    #[test]
    fn test_paste_into_empty_editor() {
        let mut app = create_app_with_editor_content(vec![]);
        app.clipboard.copy(&vec!["Hello".to_string(), "World".to_string()]);

        app.paste_selected_text().unwrap();

        assert_eq!(
            app.editor.editor_content,
            vec!["Hello".to_string(), "World".to_string()]
        );
    }


}


#[cfg(test)]
mod app_command_line_tests {
    use crate::app::*; // Access app.rs logic

    //init functions
    fn create_app_with_command_input(s: String) -> App {
        let mut app = App::new();
        app.command_line.input = s;
        app

    }

    //writing chars to command line
    #[test]
    fn test_write_char_to_command_line() {
        let mut app = create_app_with_command_input("".to_string());
        app.write_char_to_command_line('A');

        assert_eq!(app.command_line.input, "A");
        assert_eq!(app.command_line.cursor.x, 1);
    }

    #[test]
    fn test_write_char_to_command_line_mid_input() {
        let mut app = create_app_with_command_input("Test".to_string());
        app.command_line.cursor.x = 2;
        app.write_char_to_command_line('X');

        assert_eq!(app.command_line.input, "TeXst");
        assert_eq!(app.command_line.cursor.x, 3);
    }
    
    //BACKSPACE in commandline

    #[test]
    fn test_backspace_at_start() {
        let mut app = create_app_with_command_input("".to_string());
        app.command_line.cursor.x = 0;
        app.backspace_on_command_line();

        assert_eq!(app.command_line.input, "");
        assert_eq!(app.command_line.cursor.x, 0);
    }

    #[test]
    fn test_backspace_in_middle() {
        let mut app = create_app_with_command_input("Test".to_string());
        app.command_line.cursor.x = 3;
        app.backspace_on_command_line();

        assert_eq!(app.command_line.input, "Tet");
        assert_eq!(app.command_line.cursor.x, 2);
    }



}