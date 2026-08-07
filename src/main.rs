// main.rs
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);



impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}

// 必须实现 From 才能使用 ?
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError(err.to_string())
    }
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", args.path.display(), err)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}