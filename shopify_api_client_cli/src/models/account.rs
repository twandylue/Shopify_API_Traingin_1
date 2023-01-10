use super::{
    cart::Cart,
    customer::{Customer, Payment},
};

#[derive(Debug)]
pub struct Account {
    email: String,
    password: String,
    customers: Vec<Customer>,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unreachable,
    Init,
    Logined,
    SelectingProducts,
    CheckingSelectedProducts,
    CreatingCustomer,
    ReadyToPay,
    Paying,
    Paid,
}
// NOTE: mem::varient_count is not stable
const STATE_COUNT: usize = 9;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unreachable => write!(f, "Unreachable"),
            State::Init => write!(f, "Init"),
            State::Logined => write!(f, "Logined"),
            State::SelectingProducts => write!(f, "SelectingProducts"),
            State::CheckingSelectedProducts => write!(f, "CheckingSelectedProducts"),
            State::CreatingCustomer => write!(f, "CreatingCustomer"),
            State::ReadyToPay => write!(f, "ReadyToPay"),
            State::Paying => write!(f, "Paying"),
            State::Paid => write!(f, "Paid"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Unreachable => 0,
        State::Init => 1,
        State::Logined => 2,
        State::SelectingProducts => 3,
        State::CheckingSelectedProducts => 4,
        State::CreatingCustomer => 5,
        State::ReadyToPay => 6,
        State::Paying => 7,
        State::Paid => 8,
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Invalid,
    Login,
    StartToSelectProducts,
    CheckSelectedProducts,
    StartToCreateConsumer,
    CheckConsumerInfo,
    StartToPay,
    FinishPay,
}
const COMMAND_COUNT: usize = 8;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::Login => write!(f, "Login"),
            Command::StartToSelectProducts => write!(f, "StartToSelectProducts"),
            Command::CheckSelectedProducts => write!(f, "CheckSelectedProducts"),
            Command::StartToCreateConsumer => write!(f, "StartToCreateConsumer"),
            Command::CheckConsumerInfo => write!(f, "CheckConsumerInfo"),
            Command::StartToPay => write!(f, "StartToPay"),
            Command::FinishPay => write!(f, "FinishPay"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::Login => 1,
        Command::StartToSelectProducts => 2,
        Command::CheckSelectedProducts => 3,
        Command::StartToCreateConsumer => 4,
        Command::CheckConsumerInfo => 5,
        Command::StartToPay => 6,
        Command::FinishPay => 7,
    }
}

const FSM: [[State; COMMAND_COUNT]; STATE_COUNT] = [
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Logined,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::SelectingProducts,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::CheckingSelectedProducts,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::CreatingCustomer,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::ReadyToPay,
        State::Unreachable,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Paying,
        State::Unreachable,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Paid,
    ],
    [
        State::Unreachable,
        State::Unreachable,
        State::SelectingProducts,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
        State::Unreachable,
    ],
];

impl Account {
    pub fn new(email: String, password: String) -> Self {
        Account {
            email,
            password,
            customers: Vec::<Customer>::new(),
            state: self::State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let i_state = self::state_to_index(self.state);
        let i_command = self::command_to_index(command);
        let next_state = FSM[i_state][i_command];

        if next_state == State::Unreachable {
            unreachable!(
                "Invalid Account state changing. current state: {}, command: {}",
                self.state,
                command.to_string()
            );
        } else {
            self.state = next_state
        }
    }

    pub fn login(&mut self) {
        self.change_state(Command::Login);
    }

    pub fn select_products(&mut self) {
        self.change_state(Command::StartToSelectProducts);
    }

    pub fn check_selected_products(&mut self) {
        self.change_state(Command::CheckSelectedProducts);
    }

    pub fn check_cart(&mut self) {
        self.change_state(Command::StartToCreateConsumer);
    }

    pub fn add_customer(&mut self, customer: Customer) {
        // TODO: multiple customers, select by name(name should be unique)
        // let c = self.customers.iter().filter(move |e| e.name() == name);
        self.customers.push(customer);
    }

    // NOTE: return checkout url
    pub fn check_consumer(&mut self, customer_name: String) -> Customer {
        // TODO: select by name
        self.change_state(Command::CheckConsumerInfo);

        return self.customers.first().unwrap().clone();
    }

    // NOTE: clicking checkout url
    // TODO: How to know if the checkout is successful?
    pub fn pay(&mut self) {
        self.change_state(Command::StartToPay);
    }

    pub fn finish(&mut self) {
        self.change_state(Command::FinishPay);
    }

    // getters
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn customers(&self) -> Vec<Customer> {
        self.customers.clone()
    }
}
