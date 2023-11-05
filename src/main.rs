use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use rust_decimal::Decimal;
mod transaction;
use transaction::{Transaction, Operation};
mod database;
use database::DataBase;
mod balance;
use balance::ComputeBalance;
use crate::merchant::Merchant;
use crate::tag::Tag;
mod merchant;
mod tag;
mod filter;
mod display;
use crate::display::Display;


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
    /// add transaction
    Add {
        date: NaiveDate,
        #[arg(allow_hyphen_values = true)] // this allow to enter negative number without leading --
        #[arg(value_parser = check_decimal)]
        amount: Decimal,
        #[arg(short)]
        note: Option<String>,
        #[arg(short)]
        merchant: Option<String>,
        #[arg(short)]
        tag: Option<String>,
        #[arg(short)]
        account: Option<String>,
    },
    /// remove transaction by ID
    Rm {
        id: usize,
    },
    /// list all transactions
    List { },
    /// summary
    Summary { }
}

/// Check if entered amount is a decimal with only 2 number after the comma
fn check_decimal(s: &str) -> Result<Decimal, String> {
    let dec: Decimal = s.parse().map_err(|e| format!("{e}"))?;

    let dec = dec.normalize(); //Strips any trailing zero’s

    if dec.scale() > 2{
        Err("No more than 2 decimal".to_string())
    }else{
        Ok(dec)
    }
}

fn main() {
    let args = Cli::parse();

    let mut db = DataBase::load(&args.dbfile);

    match args.cmd {
        Commands::Add { date, amount , note, merchant, tag, account} => {
            let new_transaction = Transaction{id:0, date, amount, note, merchant, tag, account};
            println!("adding transaction {new_transaction:?}");
            db.add_transaction(new_transaction);
        }
        Commands::Rm { id } => { db.remove_transaction(id); }
        Commands::List { } => { db.list_transaction().display() }
        Commands::Summary { } => { summary(db.list_transaction())}
    }

    db.store(&args.dbfile);
}


fn summary(list: Vec<&Transaction>){
    let bal = list.balance();
    println!("total balance:");
    println!("{bal}");

    println!("by merchants:");
    list.merchants_balance().display();

    println!("by tags:");
    list.tags_balance().display();
}