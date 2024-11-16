 use std::io;
 
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

}
