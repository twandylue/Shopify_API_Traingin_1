use shopify_api_client_cli::models::product_list::Product_List;

pub fn render_products_info(product_list: &Product_List) {
    println!();
    println!("{}", format!("{:-<45}", ""));
    println!("Unifi 團購清單");
    let mut i = 1;
    product_list.items().iter().for_each(|product| {
        println!(
            "{}). Id: {}, name: {}, price: {}, description: {}",
            i,
            product.id(),
            product.name(),
            product.price(),
            product.description()
        );
        i += 1;
    });
    println!("{}", format!("{:-<45}", ""));
}
