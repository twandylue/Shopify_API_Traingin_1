use shopify_api_client_cli::models::{
    account::{Account, State},
    customer::{Customer, Payment},
};

#[test]
fn test_account_change_state_to_Logined_ok() {
    // arrange
    let expected = State::Logined;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);

    // act
    account.login();

    // assert
    assert_eq!(expected, account.state());
}

#[test]
#[should_panic(expected = "Invalid Account state changing. current state: Logined, command: Login")]
fn test_account_change_state_to_Logined_failed() {
    // arrange
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);

    // act
    account.login();
    account.login();

    // assert
}

#[test]
fn test_account_change_state_to_SelectingProducts_ok() {
    // arrange
    let expected = State::SelectingProducts;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);

    // act
    account.login();
    let cart = account.select_products();

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
}

#[test]
fn test_account_change_state_to_CheckingSelectedProducts_ok() {
    // arrange
    let expected = State::CheckingSelectedProducts;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);

    // act
    account.login();
    let cart = account.select_products();
    account.check_selected_products();

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
}

#[test]
fn test_account_change_state_to_CreatingConsumer_ok() {
    // arrange
    let expected = State::CreatingConsumer;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);
    let name = "andylu".to_string();
    let address = "my address".to_string();
    let payment = Payment::CreditCard;

    // act
    account.login();
    let cart = account.select_products();
    account.check_selected_products();
    account.create_consumer(name.clone(), address.clone(), payment);

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
    assert_eq!(1, account.customers().len());
    assert_eq!(
        Customer::new(name, address, payment),
        *account.customers().first().unwrap()
    );
}

#[test]
fn test_account_change_state_to_CheckingConsumerInfo_ok() {
    // arrange
    // TODO: State::CheckingConsumerInfo is unnecessary.
    let expected = State::CheckingConsumerInfo;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);
    let name = "andylu".to_string();
    let address = "my address".to_string();
    let payment = Payment::CreditCard;

    // act
    account.login();
    let cart = account.select_products();
    account.check_selected_products();
    account.create_consumer(name.clone(), address, payment);
    account.check_consumer(name.clone());

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
    assert_eq!(1, account.customers().len());
}

#[test]
fn test_account_change_state_to_Paying_ok() {
    // arrange
    let expected = State::Paying;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);
    let name = "andylu".to_string();
    let address = "my address".to_string();
    let payment = Payment::CreditCard;

    // act
    account.login();
    let cart = account.select_products();
    account.check_selected_products();
    account.create_consumer(name.clone(), address, payment);
    account.check_consumer(name.clone());
    account.pay();

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
    assert_eq!(1, account.customers().len());
}

#[test]
fn test_account_change_state_to_Paid_ok() {
    // arrange
    let expected = State::Paid;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let mut account = Account::new(email, password);
    let name = "andylu".to_string();
    let address = "my address".to_string();
    let payment = Payment::CreditCard;

    // act
    account.login();
    let cart = account.select_products();
    account.check_selected_products();
    account.create_consumer(name.clone(), address, payment);
    account.check_consumer(name.clone());
    account.pay();
    account.finish();

    // assert
    assert_eq!(expected, account.state());
    assert_eq!(cart.show_all().len(), 0);
    assert_eq!(1, account.customers().len());
}

#[test]
fn test_account_get_email_ok() {
    // arrange
    let expected = "andylu@email.com".to_string();
    let email = expected.clone();
    let password = "123456789".to_string();
    let account = Account::new(email, password);

    // act
    let actual = account.email();

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn test_account_get_state_ok() {
    // arrange
    let expected = State::Init;
    let email = "andylu@email.com".to_string();
    let password = "123456789".to_string();
    let account = Account::new(email, password);

    // act
    let actual = account.state();

    // assert
    assert_eq!(expected, actual);
}
