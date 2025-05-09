#[cfg(test)]
mod app_tests {
    use crate::app::*;
    use crate::popup::PopupResult;
    use std::fs;
    use std::path::Path;

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

    fn test_save_path(filename: &str) -> String {
        format!("test_saves/{}", filename)
    }

    #[test]
    fn test_no_pending_states_does_nothing() {
        let mut app = create_app();
        app.handle_confirmation_popup_response();
        assert!(app.pending_states.is_empty());
    }

    #[test]
    fn test_save_confirmation_saves_file_and_removes_state() {
        let mut app = create_app();
        let save_path = test_save_path("file1.txt");
        app.editor.editor_content = vec![String::from("test")];

        app.pending_states
            .push(PendingState::Saving(save_path.clone()));
        app.popup_result = PopupResult::Bool(true);

        app.handle_confirmation_popup_response();

        assert!(Path::new(&save_path).exists());
        assert!(app.pending_states.is_empty());
        assert_eq!(app.popup_result, PopupResult::None);
        assert!(app.popup.is_none());

        // Cleanup test file
        fs::remove_file(&save_path).ok();
    }

    #[test]
    fn test_save_rejection_closes_popup_but_does_not_save() {
        let mut app = create_app();
        let save_path = test_save_path("file2.txt");
        app.editor.editor_content = vec![String::from("test")];

        app.pending_states
            .push(PendingState::Saving(save_path.clone()));
        app.popup_result = PopupResult::Bool(false);

        app.handle_confirmation_popup_response();

        assert!(!Path::new(&save_path).exists());
        assert_eq!(app.popup_result, PopupResult::None);
        assert!(app.popup.is_none());
    }

    #[test]
    fn test_quit_state_calls_quit() {
        let mut app = create_app();
        app.pending_states.push(PendingState::Quitting);

        app.handle_confirmation_popup_response();

        assert!(app.pending_states.is_empty()); // Ensuring quit state was processed
    }

    #[test]
    fn test_save_then_quit_calls_save_then_quit() {
        let mut app = create_app();
        let save_path = test_save_path("file3.txt");
        app.editor.editor_content = vec![String::from("test")];
        app.pending_states
            .push(PendingState::Saving(save_path.clone()));
        app.pending_states.push(PendingState::Quitting);
        app.popup_result = PopupResult::Bool(true);

        app.handle_confirmation_popup_response();

        assert!(app.pending_states.is_empty());
        assert!(Path::new(&save_path).exists());

        // Cleanup test file
        fs::remove_file(&save_path).ok();
    }
}



#[cfg(test)]
mod app_command_line_tests {
    use crate::app::*;
    use std::fs;
    use tempfile::NamedTempFile; // Access app.rs logic

    //init functions
    fn create_app_with_editor_content(vec: Vec<String>) -> App {
        let mut app = App::new();
        app.editor.editor_content = vec;
        app
    }

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

    #[test]
    fn test_save_with_specified_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let mut app = create_app_with_editor_content(vec!["Test content".to_string()]);
        app.file_path = None;
        app.save(vec![file_path.clone(), "--force".to_string()])
            .unwrap();

        let saved_content = fs::read_to_string(file_path).unwrap();
        assert_eq!(saved_content, "Test content");
    }

    #[test]
    fn test_save_with_existing_file_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let mut app = create_app_with_editor_content(vec!["New content".to_string()]);
        app.file_path = Some(file_path.clone());
        app.save(vec![]).unwrap();

        let saved_content = fs::read_to_string(file_path).unwrap();
        assert_eq!(saved_content, "New content");
    }

    #[test]
    fn test_save_with_no_file_path_defaults_to_untitled() {
        let mut app = create_app_with_editor_content(vec!["Default content".to_string()]);

        app.save(vec![]).unwrap();

        let saved_content = fs::read_to_string("untitled").unwrap();
        assert_eq!(saved_content, "Default content");

        fs::remove_file("untitled").unwrap(); // Clean up after test
    }

    #[test]
    fn test_does_not_save_if_no_changes() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();
        fs::write(&file_path, "Unchanged content").unwrap();
        let mut app = create_app_with_editor_content(vec!["Unchanged content".to_string()]);
        app.file_path = Some(file_path.clone());

        app.save(vec![]).unwrap();

        let saved_content = fs::read_to_string(file_path).unwrap();
        assert_eq!(saved_content, "Unchanged content"); // No overwrite happened
    }

    #[test]
    fn test_save_creates_new_file_if_missing() {
        let temp_file_path = "new_test_file.txt".to_string();
        let mut app = create_app_with_editor_content(vec!["Hello World!".to_string()]);
        app.file_path = None;

        app.save(vec![temp_file_path.clone()]).unwrap();

        let saved_content = fs::read_to_string(&temp_file_path).unwrap();
        assert_eq!(saved_content, "Hello World!");

        fs::remove_file(temp_file_path).unwrap(); // Clean up
    }
}
