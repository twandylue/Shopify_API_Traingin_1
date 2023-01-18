use shopify_api_client_cli::models::{cart::Cart, product_list::Product_List};

pub fn render_cart_info(cart: &Cart, product_list: &Product_List) {
    println!();
    println!("{}", format!("{:*<45}", ""));
    println!(
        r#"
       _____           _
     / ____|         | |
    | |     __ _ _ __| |_
    | |    / _` | '__| __|
    | |___| (_| | |  | |_
     \_____\__,_|_|  \__|
    Id: {}
    "#,
        cart.id()
    );
    println!("Current Cart: ");

    cart.show_all().iter().for_each(|c| {
        if let Some(product) = product_list.items().into_iter().find(|p| p.id() == *c.0) {
            println!(
                "{}). id: {}, name: {}, price: {}, description: {}, number: {}",
                product.serial_number(),
                product.id(),
                product.name(),
                product.price(),
                product.description(),
                *c.1
            );
        } else {
            unreachable!(
                "product id(in your cart): {} is not on the product list.",
                *c.0
            );
        }
    });
    println!("{}", format!("{:*<45}", ""));
}
