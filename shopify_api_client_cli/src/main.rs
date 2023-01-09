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

#[tokio::main]
async fn main() {
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
    println!("What do you want?");
    println!("- whatsnew");
    println!("- nothing");

    let mut choose = String::new();
    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
    stdin()
        .read_line(&mut choose)
        .expect("Did not enter a correct string");

    if choose == "nothing".to_string() {
        println!("test");
        return;
    }
    println!("Unifi 團購清單");
}
