#[derive(Debug)]
pub struct Account {
    email: String,
    password: String,
    state: self::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Init,
    Logined,
}
const STATE_COUNT: usize = 2;

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Init => write!(f, "Init"),
            State::Logined => write!(f, "Logined"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Login,
}
const COMMAND_COUNT: usize = 1;

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Login => write!(f, "Login"),
        }
    }
}

fn state_to_index(state: State) -> usize {
    match state {
        State::Init => 1,
        State::Logined => 2,
    }
}

fn index_to_state(index: usize) -> State {
    let mut ret: State = State::Init;

    if index == 1 {
        ret = State::Init;
    } else if index == 2 {
        ret = State::Logined;
    } else {
        unreachable!("index: {}, undefined state machine(State).", index);
    }

    return ret;
}

fn command_to_index(command: Command) -> usize {
    match command {
        Command::Login => 1,
    }
}

fn index_to_command(index: usize) -> Command {
    let mut ret: Command = Command::Login;

    if index == 1 {
        ret = Command::Login;
    } else {
        unreachable!("index: {}, undefined state machine(Command).", index);
    }

    return ret;
}

// TODO: index of state
const FSM: [[usize; COMMAND_COUNT]; STATE_COUNT] = [[2], [0]];

impl Account {
    pub fn new(email: String, password: String) -> Self {
        Account {
            email,
            password,
            state: self::State::Init,
        }
    }

    fn change_state(&mut self, command: Command) {
        let state = self::state_to_index(self.state);
        let command = self::command_to_index(command);
        let next_state = FSM[state - 1][command - 1];

        println!("next_state: {}", next_state);
        if next_state == 0 {
            unreachable!(
                "Invalid chaning state. current state: {}, command: {}",
                self.state, command
            );
        } else {
            self.state = index_to_state(next_state);
        }
    }

    pub fn login(&mut self) {
        // todo!();
        self.change_state(Command::Login);
    }

    // getter
    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn state(&self) -> State {
        self.state
    }
}
