use crate::client::graphql_client::GraphqlClient;

use super::{customer::Customer, product::Product};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cart {
    id: String,
    checkout_url: String,
    current_products: HashMap<String, u32>,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unreachable,
    Init,
    Alive,
    Checkouted,
}
const STATE_COUNT: usize = 4;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unreachable => write!(f, "Unreachable"),
            State::Init => write!(f, "Init"),
            State::Alive => write!(f, "Alive"),
            State::Checkouted => write!(f, "Checkouted"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Unreachable => 0,
        State::Init => 1,
        State::Alive => 2,
        State::Checkouted => 3,
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Invalid,
    GetId,
    Checkout,
}
const COMMAND_COUNT: usize = 3;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::GetId => write!(f, "GetId"),
            Command::Checkout => write!(f, "Checkout"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::GetId => 1,
        Command::Checkout => 2,
    }
}

const FSM: [[State; COMMAND_COUNT]; STATE_COUNT] = [
    [State::Unreachable, State::Unreachable, State::Unreachable],
    [State::Unreachable, State::Alive, State::Checkouted],
    [State::Unreachable, State::Alive, State::Checkouted],
    [State::Unreachable, State::Unreachable, State::Unreachable],
];

impl Cart {
    pub fn new() -> Self {
        Cart {
            id: String::new(),
            checkout_url: String::new(),
            current_products: HashMap::<String, u32>::new(),
            state: State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let i_state = self::state_to_index(self.state);
        let i_command = self::command_to_index(command);
        let next_state = FSM[i_state][i_command];

        if next_state == State::Unreachable {
            unreachable!(
                "Invalid Cart state changing. current state: {}, command: {}",
                self.state,
                command.to_string()
            );
        } else {
            self.state = next_state
        }
    }

    pub fn show_all(&self) -> HashMap<String, u32> {
        self.current_products.clone()
    }

    pub fn add(&mut self, product: Product) {
        match self.state {
            State::Alive => {
                self.current_products
                    .entry(product.id())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            _ => {
                unreachable!(
                    "Invalid Cart state as adding new product into Cart. State should be {}, Current state: {}",
                    State::Alive,
                    self.state,
                );
            }
        }
    }

    pub fn remove(&mut self, product: Product) {
        match self.state {
            State::Alive => {
                self.current_products
                    .entry(product.id())
                    .and_modify(|counter| *counter -= 1);
            }
            _ => {
                unreachable!(
                    "Invalid Cart state as adding new product into Cart. State should be {}, Current state: {}",
                    State::Alive,
                    self.state,
                );
            }
        }
    }

    pub async fn get_cart_id(&mut self, access_token: String) -> (String, String) {
        self.change_state(Command::GetId);
        // NOTE: API(cartCreate with access token)
        let client = GraphqlClient::new();
        let result = client.create_cart(access_token).await;
        match result {
            Ok(response) => {
                self.id = response.0.clone();
                self.checkout_url = response.1.clone();
                return (response.0, response.1);
            }
            _ => panic!("Get cart Id is failed."),
        }
    }

    pub async fn confirm(&mut self) {
        // NOTE: API(CartLinesAdd)
        let client = GraphqlClient::new();
        match client.add_lines_to_cart(self.clone()).await {
            Ok(cart) => self.update_checkout_url(cart.checkout_url),
            Err(_) => todo!(),
        };
    }

    pub async fn update_buyer_info(&self, customer: Customer, token: String) {
        // NOTE: API(cartBuyerIdentityUpdate)
        let client = GraphqlClient::new();
        client
            .update_cart_buyer_info(customer, token, self.id.clone())
            .await;
    }

    pub fn checkout(&mut self) {
        self.change_state(Command::Checkout);
    }

    // getter
    pub fn state(&self) -> State {
        self.state
    }

    pub fn checkout_url(&self) -> String {
        match self.state {
            State::Alive => self.checkout_url.clone(),
            _ => {
                unreachable!(
                        "Invalid Cart state as getting cart checkout url. State should be {}, Current state: {}",
                        State::Alive,
                        self.state,
                );
            }
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    // setter
    pub fn update_checkout_url(&mut self, checkout_url: String) {
        self.checkout_url = checkout_url
    }
}
