mod graphqls;
mod models;

use clap::Parser;
use shopify_api_client_cli::models::account::Account;
use std::io::{stdin, stdout, Write};

#[derive(Parser, Debug)]
struct Cli {
    account: String,
    password: String,
}

fn main() {
    // let args = Cli::parse();
    println!("Hi! Please Login First");
    let _ = stdout().flush();
    println!("- Account: ");
    let mut email = String::new();
    let mut password = String::new();
    stdin()
        .read_line(&mut email)
        .expect("Did not enter a correct string");
    println!("- Password: ");
    stdin()
        .read_line(&mut password)
        .expect("Did not enter a correct string");

    let account = Account::new(email, password);
    println!();
    println!("Login Success! Hi! {}", account.email());

    println!("#####################################################");
    println!("Start Conversation...");
}
