mod cli;
mod client;
mod models;
mod render_templates;

use crate::{
    cli::cli_steps::first_step_login,
    cli::cli_steps::{
        fifth_step_creating_customers, second_step_what_do_you_want_to_do,
        sixth_step_confirm_customer_info,
    },
    cli::cli_steps::{forth_step_checking_cart, third_step_selecting_products},
};
use clap::Parser;
use shopify_api_client_cli::models::cart::Cart;

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

    let mut cart = Cart::new();
    cart.get_cart_id(account.access_token());
    third_step_selecting_products(&mut account, &mut cart);

    let mut final_cart = forth_step_checking_cart(cart, &mut account);

    fifth_step_creating_customers(&mut account);

    let final_customer = sixth_step_confirm_customer_info(&mut account);

    let checkout_url = final_cart.checkout_url();

    println!("Clicking below url to checkout.");
    println!("- Url: {}", checkout_url);

    // TODO: account paying and paid
    account.pay();
    println!("Start to Pay");
    account.finish();
    println!("Success!");
    final_cart.checkout();

    println!("end...");
}
