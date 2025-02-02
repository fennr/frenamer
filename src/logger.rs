#[derive(Clone, Copy)]
pub enum Logger {
    Verbose,
    Quiet,
}

impl Logger {
    pub fn log_rename(self, from: &str, to: &str) {
        if let Self::Verbose = self {
            println!("Renaming: {} -> {}", from, to);
        }
    }

    pub fn log_skip(self, file: &str) {
        if let Self::Verbose = self {
            println!("Skipping file: {} (no changes needed)", file);
        }
    }

    pub fn log_directory(self, dir: &str) {
        if let Self::Verbose = self {
            println!("Processing directory: {}", dir);
        }
    }
}
