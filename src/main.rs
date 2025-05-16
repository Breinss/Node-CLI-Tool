use clap::Parser;
use scan::process_cache_modules;

mod scan;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to start scanning from
    #[arg(default_value = "/")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let start_path = args.path;
    
    println!("Scanning from path: {}", start_path.display());
    
    match scan::scan(start_path) {
        Ok(paths) => {
            if let Err(e) = process_cache_modules(paths) {
                eprintln!("Error processing cache modules: {:?}", e);
            }
        },
        Err(e) => eprintln!("Error scanning directories: {:?}", e),
    }
}