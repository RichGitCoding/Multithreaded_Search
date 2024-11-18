use std::io;
use walkdir::WalkDir;
use std::fs::File;
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;


fn main() {

//Prompt use for directory to search
let mut dir_path = String::new();
println!("Please enter a directory to search through: ");
io::stdin().read_line(&mut dir_path).unwrap();
let dir_path = dir_path.trim(); //.trim() returns a string that represents a borrowed view

//Prompt user for keyword to search for
println!("Please enter a keyword to search for:");
let mut keyword = String::new();
io::stdin().read_line(&mut keyword).unwrap();
let keyword = keyword.trim().to_string(); // Trim keyword input and clone it 

//Create a channel tx and tx for communication & make safe
let (tx,rx): (mpsc::Sender<PathBuf>, mpsc::Receiver<PathBuf>) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));  

//Create vector to store thread handles
let num_threads = 4;
let mut handles: Vec<thread::JoinHandle<()>> = vec![]; 

//Spawn worker threads and clone the rx and keywords for the threads
for _ in 0..num_threads{
    let rx = Arc::clone(&rx);
    let keyword = keyword.clone();

    let handle = thread::spawn(move || {
        while let Ok(file_path) = rx.lock().unwrap().recv() {
            search_file(&file_path, &keyword); //search for the word in the file
        }
    });

    handles.push(handle);
}

//Traverse the directory
for entry in WalkDir::new(dir_path) {
    match entry {
        Ok(entry) if entry.file_type().is_file() => {
            let file_path = entry.path().to_path_buf(); // Clone the path
            if tx.send(file_path).is_err() {
                eprintln!("Error sending file path to worker");
            }
        }
        Err(e) => eprintln!("Error reading directory entry: {}", e),
        _ => {} // Ignore other cases (directories, etc.)
    }
}

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn search_file(file_path: &PathBuf, keyword: &str) {
    println!("Searching in file: {:?}", file_path); // Print the file being searched

    match File::open(file_path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let mut line_number = 1;

            for line in reader.lines() {
                match line {
                    Ok(line) if line.contains(keyword) => { // If line contains the keyword
                        println!("Found '{}' in {} (Line {}): {}", keyword, file_path.display(), line_number, line);
                    }
                    Err(_) => eprintln!("Error reading line in file: {}", file_path.display()),
                    _ => {}
                }
                line_number += 1;
            }
        }
        Err(_) => eprintln!("Error opening file: {}", file_path.display()),
    }
}





