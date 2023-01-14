use shopify_api_client_cli::models::{
    cart::{Cart, State},
    customer::{Customer, Payment},
    product::Product,
};

fn prepare_for_customer() -> Customer {
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andy@email.com".to_string();
    let phone = "123456580".to_string();
    let address = "my address".to_string();
    let payment = Payment::CreditCard;
    return Customer::new(first_name, last_name, email, phone, address, payment);
}

#[test]
fn test_cart_change_state_to_Alive_ok() {
    // arrange
    let expected = State::Alive;
    let mut cart = Cart::new();
    let p = Product::new(1, "product".to_string(), 100, "Just a product".to_string());
    let access_token = String::from("token");

    // act
    cart.get_cart_id(access_token);
    cart.add(p);

    // assert
    assert_eq!(expected, cart.state());
}

#[test]
fn test_cart_change_state_to_Checkouted_ok() {
    // arrange
    let expected = State::Checkouted;
    let mut cart = Cart::new();
    let p = Product::new(1, "product".to_string(), 100, "Just a product".to_string());
    let access_token = String::from("token");

    // act
    cart.get_cart_id(access_token);
    cart.add(p);
    cart.checkout();

    // assert
    assert_eq!(expected, cart.state());
}

#[test]
fn test_cart_get_checkout_url_ok() {
    // arrange
    let expected = State::Alive;
    let mut cart = Cart::new();
    let p = Product::new(1, "product".to_string(), 100, "Just a product".to_string());
    let access_token = String::from("token");

    // act
    cart.get_cart_id(access_token);
    cart.add(p);
    cart.checkout_url();

    // assert
    assert_eq!(expected, cart.state());
}
