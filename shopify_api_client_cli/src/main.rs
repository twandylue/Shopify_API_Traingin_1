mod cli;
mod graphqls;
mod models;
mod render_templates;

use crate::{
    cli::cli_steps::first_step_login,
    cli::cli_steps::{fifth_step_creating_consumers, second_step_what_do_you_want_to_do},
    cli::cli_steps::{forth_step_checking_cart, third_step_selecting_products},
};
use clap::Parser;

use std::io::{stdout, Write};

#[derive(Parser, Debug)]
struct Cli {
    account: String,
    password: String,
}

fn main() {
    // let args = Cli::parse();
    let _ = stdout().flush();

    let mut account = first_step_login();

    println!();
    println!("Login Success! Hi! {}", account.email());

    println!("#####################################################");
    println!("Start Conversation...");

    second_step_what_do_you_want_to_do();

    let cart = third_step_selecting_products(&mut account);

    let final_cart = forth_step_checking_cart(cart, &mut account);

    fifth_step_creating_consumers(&mut account);

    println!("end...");
}
