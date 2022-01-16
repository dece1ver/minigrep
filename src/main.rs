use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Не удалось распарсить агрументы: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка при выполнении: {}", e);
        process::exit(1);
    }    
}