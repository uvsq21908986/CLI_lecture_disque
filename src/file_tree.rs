use crate::size::Size;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, EntryNode>,
}

enum EntryNode {
    File { size: Size },
    Directory { children: Vec<PathBuf>, size: Size },
}

impl FileTree {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        let mut file_tree = FileTree {
            root: root.to_path_buf(),
            map: HashMap::new(),
        };

        file_tree
            .build_tree(root)
            .expect("Error building file tree");

        Ok(file_tree)
    }

    // Private helper function to build the file tree recursively.
    fn build_tree(&mut self, path: &Path) -> std::io::Result<Size> {
        let entry_list = read_dir(path).expect("Error reading directory");

        let mut children = Vec::new();
        let mut total_size = Size::new(0);

        for entry in entry_list {
            let entry = entry.expect("Error reading entry");
            let child_path = entry.path();

            if child_path.is_dir() {
                // Recursively build the tree for directories.
                let child_size = self.build_tree(&child_path)?;
                children.push(child_path);
                total_size = total_size + child_size;
            } else if child_path.is_file() {
                // For files, store the size in EntryNode.
                let file_size = entry.metadata().expect("Error getting metadata").len();
                self.map.insert(
                    child_path.clone(),
                    EntryNode::File {
                        size: Size::new(file_size),
                    },
                );
                children.push(child_path);
                total_size = total_size + Size::new(file_size);
            }
        }

        // Insert the EntryNode for the current directory.
        self.map.insert(
            path.to_path_buf(),
            EntryNode::Directory {
                children,
                size: total_size,
            },
        );

        Ok(total_size)
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn get_children(&self, path: &Path) -> Option<&[PathBuf]> {
        match self.map.get(path) {
            Some(EntryNode::Directory { children, .. }) => Some(children.as_slice()),
            _ => None,
        }
    }

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        match self.map.get(path) {
            Some(EntryNode::File { size }) => Some(*size),
            Some(EntryNode::Directory { size, .. }) => Some(*size),
            _ => None,
        }
    }

    // Getter method to get all files in the file tree.
    pub fn files(&self) -> Vec<PathBuf> {
        self.map
            .iter()
            .filter_map(|(path, entry)| match entry {
                EntryNode::File { .. } => Some(path.clone()),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn create_test_directory_structure() -> PathBuf {
        let root = tempfile::tempdir().expect("Error creating temp directory");

        // Create some files and subdirectories
        let file1_path = root.path().join("file1.txt");
        let file2_path = root.path().join("subdir1").join("file2.txt");
        let file3_path = root.path().join("subdir2").join("file3.txt");

        fs::create_dir_all(file2_path.parent().expect("Error getting parent directory"))
            .expect("Error creating parent directories");

        fs::create_dir_all(file3_path.parent().expect("Error getting parent directory"))
            .expect("Error creating parent directories");

        let mut file1 = File::create(&file1_path).expect("Error creating file1");
        file1
            .write_all(b"Hello, World!")
            .expect("Error writing to file1");

        let mut file2 = File::create(&file2_path).expect("Error creating file2");
        file2
            .write_all(b"Testing 123")
            .expect("Error writing to file2");

        let mut file3 = File::create(&file3_path).expect("Error creating file3");
        file3
            .write_all(b"Rust is awesome")
            .expect("Error writing to file3");

        root.into_path()
    }

    #[test]
    fn test_file_tree_creation() {
        let root_path = create_test_directory_structure();
        let file_tree = FileTree::new(&root_path).expect("Error creating file tree");

        // Add assertions to test the structure of the created file tree
        assert_eq!(file_tree.get_root(), &root_path);

        // Clean up: Remove the temporary directory
        fs::remove_dir_all(&root_path).expect("Error removing temp directory");
    }

    #[test]
    fn test_file_tree_methods() {
        let root_path = create_test_directory_structure();
        let file_tree = FileTree::new(&root_path).expect("Error creating file tree");

        // Add assertions to test various methods of the FileTree
        assert_eq!(file_tree.files().len(), 3);

        // Clean up: Remove the temporary directory
        fs::remove_dir_all(&root_path).expect("Error removing temp directory");
    }
}
