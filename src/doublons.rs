use crate::file_tree::FileTree;
use md5::{Digest, Md5};

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
};

pub fn find_duplicate_files(file_tree: &FileTree) -> io::Result<Vec<Vec<String>>> {
    // Create a HashMap to store MD5 hashes and associated file names
    let mut file_hashes: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over the files in the file tree
    for file in file_tree.files() {
        // Open the file
        let file_content = match File::open(&file) {
            Ok(content) => content,
            Err(_) => {
                // Skip to the next iteration if there's an error opening the file
                continue;
            }
        };

        // Create a buffered reader for the file
        let mut reader = BufReader::new(file_content);

        // Create a vector to store the content of the file as bytes
        let mut content = Vec::new();

        // Read the content of the file into the vector
        reader.read_to_end(&mut content)?;

        // Calculate the MD5 hash of the file content
        if let Ok(md5_hash) = calculate_md5(&content) {
            // Print the MD5 hash (for debugging purposes)

            // Retrieve the entry associated with the MD5 hash in the file_hashes map
            let entry_list = file_hashes.entry(md5_hash).or_default();

            // Push the file name (converted to a String) to the entry_list
            entry_list.push(file.display().to_string());
        }
    }

    // Filter and collect entries with more than one file (duplicates)
    let duplicate_files: Vec<Vec<String>> = file_hashes
        .values()
        .filter(|entry_list| entry_list.len() > 1)
        .cloned()
        .collect();

    Ok(duplicate_files)
}

pub fn calculate_md5(content: &Vec<u8>) -> io::Result<String> {
    let mut hasher = Md5::new();

    hasher.update(content);

    let result = hasher.finalize();
    Ok(format!("{:?}", result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_find_duplicate_files() {
        let temp_dir = tempfile::tempdir().expect("Error creating temp directory");

        // Create two identical files
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");

        let content = b"Hello, World!";
        create_file_with_content(&file1_path, content);
        create_file_with_content(&file2_path, content);

        let file_tree = FileTree::new(&temp_dir.path()).expect("Error creating file tree");

        let duplicate_files =
            find_duplicate_files(&file_tree).expect("Error finding duplicate files");

        // Print information about the actual and expected results
        println!("Actual duplicate files: {:?}", duplicate_files);

        // Expectation: Both file1.txt and file2.txt should be in the list of duplicate files
        assert!(
            duplicate_files
                .iter()
                .any(|files| files.contains(&file1_path.display().to_string())
                    && files.contains(&file2_path.display().to_string())),
            "Expected duplicate files not found"
        );

        // Clean up: Remove the temporary directory
        fs::remove_dir_all(&temp_dir).expect("Error removing temp directory");
    }

    #[test]
    fn test_calculate_md5() {
        // Create some content
        let content = b"Hello, World!";

        // Calculate the MD5 hash
        let md5_hash = calculate_md5(&content.to_vec()).expect("Error calculating MD5 hash");

        // Expectation: The MD5 hash should be a string
        assert!(!md5_hash.is_empty(), "MD5 hash is empty");

        // Clean up: No cleanup needed for this test
    }

    // Helper function to create a file with specified content
    fn create_file_with_content(file_path: &std::path::Path, content: &[u8]) {
        let mut file = File::create(file_path).expect("Error creating test file");
        file.write_all(content).expect("Error writing to test file");
    }
}
