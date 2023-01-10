use shopify_api_client_cli::models::product_list::Product_List;

pub fn render_products_info(product_list: &Product_List) {
    println!();
    println!("{}", format!("{:-<45}", ""));
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
    println!("{}", format!("{:-<45}", ""));
    println!("Please choose what's you want(id).");
}
