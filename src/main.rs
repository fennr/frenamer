use anyhow::Result;
use clap::{Parser, CommandFactory};
use std::path::PathBuf;

mod fs_utils;
mod rename;
mod logger;

use logger::Logger;

/// Утилита для переименования файлов
/// 
/// Переносит цифры в конец имени файла и заменяет специальные символы на подчеркивание.
/// Пример: "35.search-insert-position.rs" -> "search_insert_position_35.rs"
#[derive(Parser)]
#[command(
    name = "frenamer",
    version,
    about,
    long_about = "Утилита для переименования файлов.\n\
                  Правила переименования:\n\
                  1. Все цифры переносятся в конец имени файла\n\
                  2. Точки и тире заменяются на подчеркивание\n\
                  3. Расширение файла сохраняется\n\
                  \n\
                  Пример: 35.search-insert-position.rs -> search_insert_position_35.rs"
)]
struct Cli {
    /// Путь к файлу или директории для переименования
    #[arg(required = true, value_parser)]
    path: Option<PathBuf>,

    /// Показать подробную информацию о процессе
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        Cli::command().print_help().unwrap();
        std::process::exit(0);
    }

    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("Ошибка: {:#}", err);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let path = cli.path.as_ref().expect("Path is required");
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let logger = if cli.verbose {
        Logger::Verbose
    } else {
        Logger::Quiet
    };

    fs_utils::process_path(path, logger)
}
