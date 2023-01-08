#[derive(Debug)]
pub struct Account {
    email: String,
    password: String,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unreachable,
    Init,
    Logined,
}
// NOTE: mem::varient_count is not stable
const STATE_COUNT: usize = 3;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unreachable => write!(f, "Unreachable"),
            State::Init => write!(f, "Init"),
            State::Logined => write!(f, "Logined"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Unreachable => 0,
        State::Init => 1,
        State::Logined => 2,
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Invalid,
    Login,
}
const COMMAND_COUNT: usize = 2;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invalid => write!(f, "Invalid"),
            Command::Login => write!(f, "Login"),
        }
    }
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Invalid => 0,
        Command::Login => 1,
    }
}

const FSM: [[State; COMMAND_COUNT]; STATE_COUNT] = [
    [State::Unreachable, State::Unreachable],
    [State::Unreachable, State::Logined],
    [State::Unreachable, State::Unreachable],
];

impl Account {
    pub fn new(email: String, password: String) -> Self {
        Account {
            email,
            password,
            state: self::State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let i_state = self::state_to_index(self.state);
        let i_command = self::command_to_index(command);
        let next_state = FSM[i_state][i_command];

        println!("next_state: {}", next_state);
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

    // getters
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn state(&self) -> State {
        self.state
    }
}
