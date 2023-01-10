use shopify_api_client_cli::models::customer::Customer;

pub fn render_customer_info(customer: &Customer) {
    println!();
    println!("{}", format!("{:-<45}", ""));
    println!("Customer Information");
    println!("- Name: {}", customer.name());
    println!("- Address: {}", customer.address());
    println!("- Payment: {}", customer.payment());
    println!("{}", format!("{:-<45}", ""));
}
