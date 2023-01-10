use shopify_api_client_cli::models::customer::{Customer, Payment, State};

#[test]
fn test_customer_change_state_to_GotAccessToken_ok_1() {
    // arrange
    let expected = State::GotAccessToken;
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let token = customer.get_access_token();

    // assert
    assert_eq!(expected, customer.state());
}

#[test]
fn test_customer_get_first_name_ok() {
    // arrange
    let expected = "andy".to_string();
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let actual = customer.first_name();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_last_name_ok() {
    // arrange
    let expected = "lu".to_string();
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let actual = customer.last_name();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_address_ok() {
    // arrange
    let expected = "address".to_string();
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let actual = customer.address();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_payment_ok() {
    // arrange
    let expected = Payment::CreditCard;
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let actual = customer.payment();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_state_ok() {
    // arrange
    let expected = State::Init;
    let first_name = "andy".to_string();
    let last_name = "lu".to_string();
    let email = "andylu@email.com".to_string();
    let phone = "1234457599".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(first_name, last_name, email, phone, address, payment);

    // act
    let actual = customer.state();

    // assert
    assert_eq!(expected, actual);
}
