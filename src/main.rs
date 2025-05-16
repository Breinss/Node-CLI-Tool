use clap::Parser;
use scan::process_cache_modules;

mod scan;

#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}




fn main() {
    // let args = Cli::parse();
    let start_path = std::path::PathBuf::from("/");
    match scan::scan(start_path) {
        Ok(paths) => {
            if let Err(e) = process_cache_modules(paths) {
                eprintln!("Error processing cache modules: {:?}", e);
            }
        },
        Err(e) => eprintln!("Error scanning directories: {:?}", e),
    }
}