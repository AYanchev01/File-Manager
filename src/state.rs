pub struct AppState {
    pub last_key_pressed: Option<char>,
    pub last_modifier: Option<crossterm::event::KeyModifiers>,
    pub was_cut: bool,
    pub terminal_height: usize,
    pub delete_mode: bool,
    pub rename_mode: bool,
    pub renaming_buffer: Option<String>,
    pub prompt_message: Option<String>,
    pub search_pattern: Option<String>,
    pub search_mode: bool,
    pub last_search_index: Option<usize>,
    pub is_creating_file: bool,
    pub is_creating_directory: bool,
    pub selected_file_for_copy: Option<std::path::PathBuf>,
    pub creation_buffer: Option<String>,
    pub is_changing_permissions: bool,
    pub permissions_buffer: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        // Calculate the half screen size
        let terminal_size = crossterm::terminal::size().unwrap();

        AppState {
            last_key_pressed: None,
            last_modifier: None,
            was_cut: false,
            terminal_height : (terminal_size.1 as usize - 4) * 90 / 100,
            delete_mode: false,
            rename_mode: false,
            renaming_buffer: None,
            prompt_message: None,
            search_pattern: None,
            search_mode: false,
            last_search_index: None,
            is_creating_file: false,
            is_creating_directory: false,
            selected_file_for_copy: None,
            creation_buffer: None,
            is_changing_permissions: false,
            permissions_buffer: None,
        }
    }
}