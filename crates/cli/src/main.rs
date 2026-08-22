use clap::{Parser, Subcommand};
#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Initialize a new repository")]
    Init,
    #[command(about = "Check the repository")]
    Check,
}
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => println!("Initialization"),
        Command::Check => println!("Checking"),
    }
}
