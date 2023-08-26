#[cfg(test)]
mod tests {
    use file_manager::fs_utils::*;
    use tempfile::tempdir;
    use std::{fs::{File, self}, path::PathBuf};
    use tui::widgets::ListState;

    #[test]
    fn test_update_selected_dir() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let files = vec![
            FileInfo {
                name: "test.txt".into(),
                perms: None,
                is_dir: false,
                is_exec: false,
            },
        ];

        let current_dir = dir.path().to_path_buf();
        let mut selected_dir = PathBuf::new();
        let mut middle_state = ListState::default();
        middle_state.select(Some(0));
        let mut scroll_position = 0;

        update_selected_dir(&files, &current_dir, &mut selected_dir, &middle_state, &mut scroll_position);
        assert_eq!(selected_dir, file_path);
    }

    #[test]
    fn test_fetch_children_for_empty_dir() {
        let dir = tempdir().unwrap();
        let children = fetch_children(&dir.path().to_path_buf(), 0, 5);

        assert_eq!(children.0[0].name, "empty");
        assert_eq!(children.1, 0);
    }

    #[test]
    fn test_fetch_children_for_dir_with_files() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("test.txt")).unwrap();

        let children = fetch_children(&dir.path().to_path_buf(), 0, 5);
        assert_eq!(children.0[0].name, "test.txt");
    }

    #[test]
    fn test_make_unique_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let file_path_2 = dir.path().join("test_1.txt");
        File::create(&file_path).unwrap();

        let unique_path = make_unique_path(file_path.clone());
        assert_eq!(unique_path, file_path_2);
    }

    #[test]
    fn test_copy_file() {
        let dir_src = tempdir().unwrap();
        let dir_dest = tempdir().unwrap();

        let file_src = dir_src.path().join("test.txt");
        File::create(&file_src).unwrap();

        let file_dest = dir_dest.path().join("test.txt");

        copy(&file_src, &file_dest).unwrap();
        assert!(file_dest.exists());
    }

    #[test]
    fn test_delete_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        delete(&file_path).unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_move_file() {
        let dir_src = tempdir().unwrap();
        let dir_dest = tempdir().unwrap();

        let file_src = dir_src.path().join("test.txt");
        File::create(&file_src).unwrap();

        let file_dest = dir_dest.path().join("test_moved.txt");

        move_file(&file_src, &file_dest).unwrap();
        assert!(!file_src.exists());
        assert!(file_dest.exists());
    }

    #[test]
    fn test_get_files_and_dirs() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("test1.txt")).unwrap();
        File::create(dir.path().join("test2.txt")).unwrap();
        fs::create_dir(dir.path().join("testdir")).unwrap();

        let items = get_files_and_dirs(dir.path());
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|item| item.name == "test1.txt"));
        assert!(items.iter().any(|item| item.name == "test2.txt"));
        assert!(items.iter().any(|item| item.name == "testdir"));
    }

    #[test]
    fn test_get_parent_content() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        File::create(subdir.join("test.txt")).unwrap();

        let parent_content = get_parent_content(&subdir);
        assert_eq!(parent_content.len(), 1);
        assert_eq!(parent_content[0].name, "subdir");
    }

    #[test]
    fn test_create_file_info() {
        let name = "test_file".to_string();
        let info = create_file_info(name.clone());
        assert_eq!(info.name, name);
        assert_eq!(info.is_dir, false);
    }

    #[test]
    fn test_copy_dir_to() {
        let src_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();

        let sub_dir = src_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        File::create(sub_dir.join("test.txt")).unwrap();

        copy_dir_to(src_dir.path(), dest_dir.path()).unwrap();

        assert!(dest_dir.path().join("subdir").exists());
        assert!(dest_dir.path().join("subdir/test.txt").exists());
    }

    #[test]
    fn test_delete_dir() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        delete(&subdir).unwrap();
        assert!(!subdir.exists());
    }
}