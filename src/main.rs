 use std::io;
 use walkdir::WalkDir;
 
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


    println!("Searching for '{}' in '{}'",keyword, dir_path);

    //Traverse the Directory and print all file paths
    for entry in WalkDir::new(dir_path)
        .into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                println!("Found file: {}", entry.path().display());
            } 
        }

}
