mod graphqls;
mod models;

use clap::Parser;
use shopify_api_client_cli::models::{
    account::{Account, State},
    cart::Cart,
    product_list::Product_List,
};
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

    let mut account = Account::new(email, password);
    account.login();

    println!();
    println!("Login Success! Hi! {}", account.email());

    println!("#####################################################");
    println!("Start Conversation...");
    println!("What do you want to do?");
    println!("- whatsnew");
    println!("- nothing");

    let mut decision = String::new();
    stdin()
        .read_line(&mut decision)
        .expect("Did not enter a correct string");

    // println!("decision: {}", decision.trim_end());
    // println!("result: {}", decision.trim_end().eq("wahtsnew"));

    if !decision.trim_end().eq("whatsnew") {
        println!("Ok! I can do nothing. ByeBye!");
        return;
    }

    let list = Product_List::new();
    // let cart: Cart = account.select_products();
    account.select_products();
    while account.state() == State::SelectingProducts {
        print_products_template(&list);
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if input.trim_end().eq("x") {
            account.check_selected_products();
            break;
        }

        let ele = list
            .items()
            .into_iter()
            .find(|x| x.id() == input.trim_end().parse::<u32>().unwrap());

        println!("p: {:?}", ele.unwrap());

        // cart.add();
    }

    println!("end...");
}

pub fn print_products_template(product_list: &Product_List) {
    println!("Unifi 團購清單");
    product_list.items().iter().for_each(|product| {
        println!(
            "{}). name: {}, price: {}, description: {}",
            product.id(),
            product.name(),
            product.price(),
            product.description()
        );
    });
    println!("----------------------------------");
    println!("Please choose what's you want(id).");
    println!("input 'x' to checkout");
}
