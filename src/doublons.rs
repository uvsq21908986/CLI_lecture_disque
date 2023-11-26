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
            let entry_list = file_hashes.entry(md5_hash).or_insert(Vec::new());

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

    hasher.update(&content);

    let result = hasher.finalize();
    Ok(format!("{:?}", result))
}
