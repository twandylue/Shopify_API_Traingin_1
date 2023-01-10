use shopify_api_client_cli::models::{
    cart::{Cart, State},
    product::Product,
};

#[test]
fn test_cart_change_state_to_Alive_ok() {
    // arrange
    let expected = State::Alive;
    let mut cart = Cart::new();
    let p = Product::new(1, "product".to_string(), 100, "Just a product".to_string());

    // act
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

    // act
    cart.add(p);
    cart.checkout();

    // assert
    assert_eq!(expected, cart.state());
}
