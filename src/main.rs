use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use rust_decimal::Decimal;
mod transaction;
use transaction::Transaction;
mod database;
use database::DataBase;

#[derive(Parser, Debug)]
struct Cli {
    // command
    #[command(subcommand)]
    cmd: Commands,
    /// Sets a default database file
    #[arg(
        long,
        value_name = "database file",
        default_value = "money.db",
        global = true
    )] // allow to use --db after subcommand
    dbfile: std::path::PathBuf,
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

    let mut db = DataBase::load(&args.dbfile);

    match args.cmd {
        Commands::Add { date, amount } => {
            let new_transaction = Transaction{id:0, date, amount};
            println!("adding transaction {new_transaction:?}");
            db.add_transaction(new_transaction);
        }
    }

    db.store(&args.dbfile);
}
