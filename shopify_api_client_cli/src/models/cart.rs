use std::collections::HashMap;

use super::product::Product;

#[derive(Debug, Clone)]
pub struct Cart {
    id: String,
    checkout_url: String,
    current_products: HashMap<u32, u32>,
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
    AddOrRemoveProducts,
    Checkout,
}
const COMMAND_COUNT: usize = 3;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::AddOrRemoveProducts => write!(f, "AddOrRemoveProducts"),
            Command::Checkout => write!(f, "Checkout"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::AddOrRemoveProducts => 1,
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
        // TODO: API(cartCreate)
        Cart {
            id: String::new(),
            checkout_url: String::new(),
            current_products: HashMap::<u32, u32>::new(),
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

    pub fn show_all(&self) -> HashMap<u32, u32> {
        self.current_products.clone()
    }

    pub fn add(&mut self, product: Product) {
        self.change_state(Command::AddOrRemoveProducts);
        self.current_products
            .entry(product.id())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    pub fn remove(&mut self, product: Product) {
        self.change_state(Command::AddOrRemoveProducts);
        self.current_products
            .entry(product.id())
            .and_modify(|counter| *counter -= 1);
    }

    pub fn checkout(&mut self) -> String {
        self.change_state(Command::Checkout);
        // TODO: API(checkoutCreate)

        return self.checkout_url.clone();
    }

    // getter
    pub fn state(&self) -> State {
        self.state
    }
}
