use shopify_api_client_cli::models::customer::{Customer, Payment, State};

#[test]
fn test_customer_change_state_to_ReadyToCheck_ok_1() {
    // arrange
    let expected = State::ReadyToCheck;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.set_checkout_url("checkout_url".to_string());

    // assert
    assert_eq!(expected, customer.state());
}

#[test]
fn test_customer_change_state_to_ReadyToCheck_ok_2() {
    // arrange
    let expected = State::ReadyToCheck;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.set_checkout_url("checkout_url".to_string());
    customer.checkout();
    customer.set_checkout_url("checkout_url".to_string());

    // assert
    assert_eq!(expected, customer.state());
}

#[test]
fn test_customer_change_state_to_ReadyToCheck_ok_3() {
    // arrange
    let expected = State::ReadyToCheck;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.set_checkout_url("checkout_url".to_string());
    customer.set_checkout_url("checkout_url".to_string());

    // assert
    assert_eq!(expected, customer.state());
}

#[test]
fn test_customer_change_state_to_Checkouted_ok() {
    // arrange
    let expected = State::Checkouted;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.set_checkout_url("checkout_url".to_string());
    customer.checkout();

    // assert
    assert_eq!(expected, customer.state());
}

#[test]
#[should_panic(
    expected = "Invalid Customer state changing. current state: Init, command: Checkout"
)]
fn test_customer_change_state_to_Checkouted_failed_1() {
    // arrange
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.checkout();

    // assert
}

#[test]
#[should_panic(
    expected = "Invalid Customer state changing. current state: Checkouted, command: Checkout"
)]
fn test_customer_change_state_to_Checkouted_failed_2() {
    // arrange
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let mut customer = Customer::new(name, address, payment);

    // act
    customer.set_checkout_url("url".to_string());
    customer.checkout();
    customer.checkout();

    // assert
}

#[test]
fn test_customer_get_name_ok() {
    // arrange
    let expected = "andylu".to_string();
    let name = expected.clone();
    let address = "address".to_string();
    let payment = Payment::CreditCard;
    let customer = Customer::new(name, address, payment);

    // act
    let actual = customer.name();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_address_ok() {
    // arrange
    let expected = "address".to_string();
    let name = "andylu".to_string();
    let address = expected.clone();
    let payment = Payment::CreditCard;
    let customer = Customer::new(name, address, payment);

    // act
    let actual = customer.address();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_payment_ok() {
    // arrange
    let expected = Payment::CreditCard;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = expected.clone();
    let customer = Customer::new(name, address, payment);

    // act
    let actual = customer.payment();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_customer_get_state_ok() {
    // arrange
    let expected = State::Init;
    let name = "andylu".to_string();
    let address = "address".to_string();
    let payment = Payment::PickUPAtShop;
    let customer = Customer::new(name, address, payment);

    // act
    let actual = customer.state();

    // assert
    assert_eq!(expected, actual);
}
