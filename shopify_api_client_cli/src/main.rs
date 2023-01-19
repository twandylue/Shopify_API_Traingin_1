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
use shopify_api_client_cli::{
    client::graphql_client::GraphqlClient,
    models::{cart::Cart, product_list::Product_List},
};

use std::{
    error::Error,
    io::{stdout, Write},
};

#[derive(Parser, Debug)]
struct Cli {
    account: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let args = Cli::parse();
    let _ = stdout().flush();

    let mut account = first_step_login().await;

    println!();
    println!("Login Success! Hi! {}", account.email());

    println!("#####################################################");
    println!("Start Conversation...");

    second_step_what_do_you_want_to_do();

    let mut cart = Cart::new();
    cart.get_cart_id(account.access_token()).await;

    let mut product_list = Product_List::new();
    product_list.dowload_products().await;

    third_step_selecting_products(&mut account, &mut cart, &product_list).await;

    let final_cart = forth_step_checking_cart(cart, &mut account, &product_list).await;

    fifth_step_creating_customers(&mut account);

    let final_customer = sixth_step_confirm_customer_info(&mut account);

    final_cart
        .update_buyer_info(final_customer, account.access_token())
        .await;

    let checkout_url = final_cart.checkout_url();

    println!("Clicking below url to checkout.");
    println!("- Url: {}", checkout_url);

    println!("Thank you. Byebye!");

    return Ok(());
}
