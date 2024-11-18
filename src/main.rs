 use std::io;
 use walkdir::WalkDir;
 use std::fs::File;
 use std::io::BufRead;
 use std::sync::mpsc;
 use std::thread;
 use std::sync::{Arc, Mutex};
 
 fn main() {
    println!("Enter directory path:");
    
    let mut dir_path = String::new();
    io::stdin().read_line(&mut dir_path).unwrap();
    let dir_path = dir_path.trim();


    //Enter the keywprd to search for
    println!("Enter the keyword to search for:");

    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).unwrap();
    let keyword = keyword.trim();

    //Set up channel to communicate between threads
    let (tx,rx) = mpsc::channel();

    //Allow rx to safely share values
    let rx = Arc::new(Mutex::new(rx));

    println!("Searching for '{}' in '{}'",keyword, dir_path);

    // Traverse the directory and search each file
    for entry in WalkDir::new(dir_path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let file_path = entry.path();
                    println!("Searching in file: {}", file_path.display());

                    // Open the file
                    let file = File::open(file_path);
                    match file {
                        Ok(file) => {
                            let reader = io::BufReader::new(file);
                            let mut line_number = 1;

                            // Iterate over each line in the file
                            for line in reader.lines() {
                                match line {
                                    Ok(line) => {
                                        if line.contains(keyword) {
                                            println!("Found in {} (Line {}): {}", file_path.display(), line_number, line);
                                        }
                                    }
                                    Err(_) => eprintln!("Error reading line in file: {}", file_path.display()),
                                }
                                line_number += 1;
                            }
                        }
                        Err(_) => eprintln!("Error opening file: {}", file_path.display()),
                    }
                }
            }
            Err(_) => eprintln!("Error reading entry in directory: {}", dir_path),
        }
    }
}
// START program

// PROMPT user for directory path and keyword to search

// INITIALIZE a thread-safe queue to store file paths
// INITIALIZE a channel for threads to send search results to the main thread

// SPAWN a thread for directory traversal:
//     RECURSIVELY traverse the given directory
//     FOR each file found:
//         ADD file path to the queue

// SPAWN multiple worker threads:
//     WHILE queue is not empty:
//         POP a file path from the queue
//         READ the file contents
//         SEARCH for the keyword in each line
//         SEND results (file name and line numbers) back to the main thread via channel

// COLLECT results in the main thread:
//     DISPLAY the file names and line numbers where the keyword was found

// WAIT for all threads to finish

// END program
