use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use rust_decimal::Decimal;


#[derive(Parser, Debug)]
struct Cli {
    // command
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        date: NaiveDate,
        amount: Decimal,
    },
}

fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Add { date, amount } => { println!("adding transaction of {amount} on {date}") }
    }
}
