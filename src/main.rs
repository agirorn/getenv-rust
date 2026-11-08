use clap::Parser;
use std::env;
use std::process::ExitCode;

#[derive(Parser)]
#[command(about)]
struct Cli {
    /// Enviroment variable name
    name: Option<String>,

    /// Support case insesitive variable name
    #[arg(short = 'i', long, default_value_t = false)]
    case_insensitive: bool,

    /// List enviroment variable names
    #[arg(short = 'l', long, default_value_t = false)]
    list: bool,
}

fn main() -> ExitCode {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    if args.list {
        for (name, _) in env::vars() {
            println!("{name}");
        }
        return ExitCode::SUCCESS;
    }

    match args.name {
        None => {
            for (name, value) in env::vars() {
                println!("{name}={value}");
            }
        }
        Some(mut name) => {
            if args.case_insensitive {
                name = name.to_uppercase();
            }
            match env::var(&name) {
                Ok(value) => println!("{value}"),
                Err(_) => {
                    eprintln!("Environment variable ({name}) not found");
                    return ExitCode::from(1);
                }
            }
        }
    };
    ExitCode::SUCCESS
}
