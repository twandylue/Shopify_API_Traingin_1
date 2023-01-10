use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    password: String,
    address: String,
    payment: Payment,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payment {
    CreditCard,
    PickUpAtShop,
}

impl Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Payment::CreditCard => write!(f, "CreditCart"),
            Payment::PickUpAtShop => write!(f, "PickUpAtShop"),
        }
    }
}

impl FromStr for Payment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "creditcard" => Ok(Payment::CreditCard),
            "CreditCard" => Ok(Payment::CreditCard),
            "PickUpAtShop" => Ok(Payment::PickUpAtShop),
            "pickupatshop" => Ok(Payment::PickUpAtShop),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unreachable,
    Init,
    GotAccessToken,
}
// NOTE: std::mem::varient_count is not stable
const STATE_COUNT: usize = 3;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unreachable => write!(f, "Unreachable"),
            State::Init => write!(f, "Init"),
            State::GotAccessToken => write!(f, "GotAccessToken"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Unreachable => 0,
        State::Init => 1,
        State::GotAccessToken => 2,
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Invalid,
    GetAccessToken,
}
const COMMAND_COUNT: usize = 2;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::GetAccessToken => write!(f, "SetCheckoutUrl"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::GetAccessToken => 1,
    }
}

const FSM: [[State; COMMAND_COUNT]; STATE_COUNT] = [
    [State::Unreachable, State::Unreachable],
    [State::Unreachable, State::GotAccessToken],
    [State::Unreachable, State::GotAccessToken],
];

impl Customer {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
        address: String,
        payment: Payment,
    ) -> Self {
        // TODO: API(customerCreate)
        Customer {
            id: String::new(),
            first_name,
            last_name,
            email,
            phone,
            password: "000000000".to_string(),
            address,
            payment,
            state: State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let i_state = self::state_to_index(self.state);
        let i_command = self::command_to_index(command);
        let next_state = FSM[i_state][i_command];

        if next_state == State::Unreachable {
            unreachable!(
                "Invalid Customer state changing. current state: {}, command: {}",
                self.state,
                command.to_string()
            );
        } else {
            self.state = next_state
        }
    }

    pub fn get_access_token(&mut self) -> String {
        // TODO: API(customerAccessTokenCreate)
        let a = String::new();
        self.change_state(Command::GetAccessToken);

        return a;
    }

    // pub fn checkout_cart(&mut self, cart: Cart) {
    //     // TODO: API(checkout)
    //     todo!();
    // }

    // getters
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn payment(&self) -> Payment {
        self.payment
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn state(&self) -> State {
        self.state
    }
}
