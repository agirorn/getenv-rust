use clap::Parser;
use dotenv;
use std::env;

#[derive(Parser)]
struct Cli {
    /// This will be the first positional argument
    name: String,

    #[arg(short = 'i', long, default_value_t = false)]
    case_insensitive: bool,
}

use std::process::ExitCode;

fn main() -> ExitCode {
    // fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    let mut name: String = args.name.clone();
    if args.case_insensitive {
        name = name.to_uppercase();
    }

    match env::var(&name) {
        Ok(value) => {
            // let value = env::var(&name).expect("Environment variable ({name}) not found");
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!("Environment variable ({name}) not found");
            ExitCode::from(1)
        }
    }
    // // let value = env::var(&name).expect("Environment variable ({name}) not found");
    // // println!("{value}");
    // ExitCode::SUCCESS
}
