use shopify_api_client_cli::models::{
    account::{Account, State},
    cart::Cart,
    customer::{Customer, Payment},
    product_list::Product_List,
};
use std::{io::stdin, str::FromStr};

use crate::render_templates::{
    render_cart_templates, render_customer_templates, render_products_templates,
};

pub async fn first_step_login() -> Account {
    println!("Hi! Please Login First");
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
    account.login().await;

    return account;
}

pub fn second_step_what_do_you_want_to_do() {
    println!("What do you want to do?");
    println!("- whatsnew");
    println!("- nothing");

    let mut decision = String::new();
    stdin()
        .read_line(&mut decision)
        .expect("Did not enter a correct string");

    if !decision.trim_end().eq("whatsnew") {
        println!("Ok! I can do nothing. ByeBye!");
        return;
    }
}

pub fn third_step_selecting_products(account: &mut Account, cart: &mut Cart) {
    let list = Product_List::new();
    account.select_products();
    while account.state() == State::SelectingProducts {
        render_products_templates::render_products_info(&list);
        println!("Please choose what's you want(product Id).");
        println!("input 'x' to check your cart");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if input.trim_end().eq("x") {
            account.check_selected_products();
            break;
        }

        if let Ok(id) = input.trim_end().parse::<u32>() {
            match list.items().into_iter().find(|x| x.id() == id) {
                Some(product) => cart.add(product),
                None => println!(
                "Your input is not match the current product Id, please select the product again."
            ),
            }
        } else {
            println!(
                "Your input is not match the current product Id, please select the product again."
            );
        }

        render_cart_templates::render_cart_info(&cart);
    }
}

pub fn forth_step_checking_cart(cart: Cart, account: &mut Account) -> Cart {
    let mut final_cart = cart;
    while account.state() == State::CheckingSelectedProducts {
        println!("Checking your personal cart...");
        render_cart_templates::render_cart_info(&final_cart);
        println!("Please input item number to 'remove' the product from your personal cart.");
        println!("Or input 'x' to confirm your personal cart.");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if input.trim_end().eq("x") {
            account.check_cart();
            final_cart.confirm();
            break;
        }

        let item = input.trim_end().parse::<u32>().unwrap();

        let product_list = Product_List::new();
        if let Some(product) = product_list.items().into_iter().find(|p| p.id() == item) {
            final_cart.remove(product);
        } else {
            unreachable!(
                "product id(in your cart): {} is not on the product list.",
                item
            );
        }
    }

    return final_cart;
}

pub fn fifth_step_creating_customers(account: &mut Account) {
    println!("Creating Customers...");

    while account.state() == State::CreatingCustomer {
        println!("Input the following information for new customer: ");
        println!("- First Name: ");
        let mut first_name = String::new();
        stdin()
            .read_line(&mut first_name)
            .expect("Did not enter a correct string");

        println!("- Last Name: ");
        let mut last_name = String::new();
        stdin()
            .read_line(&mut last_name)
            .expect("Did not enter a correct string");

        println!("- Address: ");
        let mut address = String::new();
        stdin()
            .read_line(&mut address)
            .expect("Did not enter a correct string");

        println!("- eamil: ");
        let mut email = String::new();
        stdin()
            .read_line(&mut email)
            .expect("Did not enter a correct string");

        println!("- phone: ");
        let mut phone = String::new();
        stdin()
            .read_line(&mut phone)
            .expect("Did not enter a correct string");

        println!("- Payment(CreditCard/PickUpAtShop): ");
        let mut payment = String::new();
        stdin()
            .read_line(&mut payment)
            .expect("Did not enter a correct string");

        let customer = Customer::new(
            first_name.trim_end().to_string(),
            last_name.trim_end().to_string(),
            email.trim_end().to_string(),
            phone.trim_end().to_string(),
            address.trim_end().to_string(),
            Payment::from_str(&payment.trim_end()).unwrap(),
        );
        account.add_customer(customer.clone());

        // TODO: multiple customers
        // println!();

        render_customer_templates::render_customer_info(&customer);
        println!("Please input 'x' to stop creating new customers.");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if input.trim_end().eq("x") {
            break;
        }
    }
}

pub fn sixth_step_confirm_customer_info(account: &mut Account) -> Customer {
    while account.state() == State::CreatingCustomer {
        println!("Current Customer:");
        account.customers().iter().for_each(|customer| {
            render_customer_templates::render_customer_info(&customer);
        });

        println!("Please confirm your customer information...");
        println!("If the customer information is correct, please input 'x'");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if input.trim_end().eq("x") {
            // TODO: let user select customers
            let mut customer = account.customers().into_iter();
            match customer.next() {
                Some(c) => {
                    account.check_consumer(c.id());
                    return c;
                }
                None => unreachable!("Customer Id does not exist."),
            }
        }
    }

    unreachable!("You have to comfirm your Customer information.");
}
