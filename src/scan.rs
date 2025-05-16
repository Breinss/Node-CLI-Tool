use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use scan_dir::ScanDir;

#[derive(Debug)]
pub enum MyError {
    Scan(scan_dir::Error),
    File(io::Error, PathBuf),
}

/// Scans directories and files from the home directory.
/// Returns a Result type that contains either:
/// - Ok(()): Indicating successful completion with no return value
/// - Err(MyError): An error that occurred during scanning
///     pub fn scan(scan_path: PathBuf) {

pub fn scan(scan_path: PathBuf) -> Result<Vec<PathBuf>, MyError> {
    let mut cache_modules_paths = Vec::new();
    
    // Get the current user's home directory as the starting point for scanning
    // unwrap_or provides a fallback ("./" current directory) if home_dir() returns None
    // This is a common Rust pattern for handling Optional values
    
    
    // Start a directory scan operation using the ScanDir library
    // The "all()" method indicates we want both files and directories
    // The read() function performs the actual scan operation on scan_path
    let result = ScanDir::all().read(scan_path, |iter| {
        // This is a closure (anonymous function) that processes the iterator
        // The 'iter' parameter contains pairs of (DirEntry, filename)
        for (entry, _name) in iter {
            // For each entry, determine if it's a directory or file
            // map() is used to transform the Option<FileType> returned by file_type()
            // unwrap_or() provides a default value (false) if file_type() fails 
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            if is_dir {
                let is_node_modules = entry.path().file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == "node_modules")
                .unwrap_or(false);
                
                if is_node_modules {
                    println!("Found node_modules: {:?}", entry.path());
                    let is_in_cache = entry.path().ancestors().any(|ancestor| {
                        ancestor.file_name()
                            .and_then(|name| name.to_str())
                            .map(|name| name == ".cache")
                            .unwrap_or(false)
                    });
                    
                    if is_in_cache {
                        // Add this path to our collection
                        cache_modules_paths.push(entry.path().to_path_buf());
                    }
                } else {
                    // Regular directory, recurse as before
                    match scan(entry.path()) {
                        Ok(mut sub_paths) => cache_modules_paths.append(&mut sub_paths),
                        Err(_) => {} // Optionally handle errors
                    }
                }
            }
        }
        // Return Ok(()) from the closure, indicating successful processing
        // This matches the requirement of the read() function
        Ok(())
    })
    // The following chain handles errors at different stages:
    
    // map_err() transforms a scan_dir::Error into our custom MyError type
    // This is how Rust encourages proper error type conversion
    .map_err(MyError::Scan)
    
    // and_then() only executes if the previous operation succeeded
    // It passes the successful value from ScanDir::read() through (which itself returns a Result)
    .and_then(|val| val);
    
    // Error handling - print any errors that occurred but continue execution
    // The 'ref' keyword borrows the error value without consuming it
    if let Err(ref e) = result {
        // println!("Error occurred: {:?}", e);
    }
    
    // Use the ? operator to propagate errors
    // If result is Ok, continue execution
    // If result is Err, return early with the error
    result?;
    
    // Since we've passed the ? operator check, we can return the collected paths
    Ok(cache_modules_paths)
}

pub fn process_cache_modules(paths: Vec<PathBuf>) -> Result<(), MyError> {
    if paths.is_empty() {
        println!("No node_modules directories found in .cache");
        return Ok(());
    }

    println!("Found {} node_modules directories in .cache:", paths.len());
    for (i, path) in paths.iter().enumerate() {
        println!("{}: {:?}", i + 1, path);
    }

    print!("Do you want to delete these node_modules and their parent directories? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| MyError::File(e, PathBuf::new()))?;
    
    if input.trim().to_lowercase() == "y" {
        for path in paths {
            // Get the parent directory of node_modules
            if let Some(parent) = path.parent() {
                println!("Deleting: {:?}", parent);
                match fs::remove_dir_all(parent) {
                    Ok(_) => println!("Successfully deleted {:?}", parent),
                    Err(e) => println!("Failed to delete {:?}: {}", parent, e),
                }
            } else {
                println!("Couldn't determine parent for {:?}", path);
                // If can't get parent, delete just the node_modules
                match fs::remove_dir_all(&path) {
                    Ok(_) => println!("Deleted node_modules only: {:?}", path),
                    Err(e) => println!("Failed to delete {:?}: {}", path, e),
                }
            }
        }
    } else {
        println!("Operation cancelled");
    }
    
    Ok(())
}

