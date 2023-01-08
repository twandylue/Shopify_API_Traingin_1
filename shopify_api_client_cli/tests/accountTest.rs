use shopify_api_client_cli::models::account::{Account, State};

#[test]
fn test_account_login_change_state_successful() {
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
