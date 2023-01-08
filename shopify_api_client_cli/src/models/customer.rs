#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    name: String,
    address: String,
    payment: Payment,
    checkout_url: Option<String>,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payment {
    CreditCard,
    PickUPAtShop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unreachable,
    Init,
    ReadyToCheck,
    Checkouted,
}
// NOTE: std::mem::varient_count is not stable
const STATE_COUNT: usize = 4;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unreachable => write!(f, "Unreachable"),
            State::Init => write!(f, "Init"),
            State::ReadyToCheck => write!(f, "ReadyToCheck"),
            State::Checkouted => write!(f, "Checkouted"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Unreachable => 0,
        State::Init => 1,
        State::ReadyToCheck => 2,
        State::Checkouted => 3,
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Invalid,
    SetCheckoutUrl,
    Checkout,
}
const COMMAND_COUNT: usize = 3;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::SetCheckoutUrl => write!(f, "SetCheckoutUrl"),
            Command::Checkout => write!(f, "Checkout"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::SetCheckoutUrl => 1,
        Command::Checkout => 2,
    }
}

const FSM: [[State; COMMAND_COUNT]; STATE_COUNT] = [
    [State::Unreachable, State::Unreachable, State::Unreachable],
    [State::Unreachable, State::ReadyToCheck, State::Unreachable],
    [State::Unreachable, State::ReadyToCheck, State::Checkouted],
    [State::Unreachable, State::ReadyToCheck, State::Unreachable],
];

impl Customer {
    pub fn new(name: String, address: String, payment: Payment) -> Self {
        Customer {
            name,
            address,
            payment,
            checkout_url: None,
            state: State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let i_state = self::state_to_index(self.state);
        let i_command = self::command_to_index(command);
        let next_state = FSM[i_state][i_command];

        println!("next_state: {}", next_state);
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

    pub fn set_checkout_url(&mut self, url: String) {
        self.checkout_url = Some(url);
        self.change_state(Command::SetCheckoutUrl);
    }

    pub fn checkout(&mut self) -> String {
        let pay_url = String::new();
        // TODO: checkout
        // todo!();
        self.change_state(Command::Checkout);

        return pay_url;
    }

    // getters
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn payment(&self) -> Payment {
        self.payment
    }

    pub fn state(&self) -> State {
        self.state
    }
}
