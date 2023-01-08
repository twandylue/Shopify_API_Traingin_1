use shopify_api_client_cli::models::account::{Account, State};

#[test]
fn test_account_login_change_state_ok() {
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
fn test_account_get_name_ok() {
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
