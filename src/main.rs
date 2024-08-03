
use clap::Parser;

#[derive(Parser)] /// derive marko that auto implements the (default-)methods of the Parser trait
#[command(version="0.0.0", long_about="My test application for CLI tools")]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args: Cli = Cli::parse();
    println!("pattern = {:?}, path = {:?}", args.pattern, args.path);
}
