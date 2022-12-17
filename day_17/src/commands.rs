pub struct Commands {
    current: usize,
    command_list: String,
}

impl Commands {
    pub fn new(list: &str) -> Self {
        Self {
            current: 0,
            command_list: String::from(list),
        }
    }

    pub fn next(&mut self) -> char {
        let next = self.current % self.command_list.len();
        let char = self.command_list.chars().nth(next).unwrap();
        self.current += 1;

        char
    }
}

pub fn create(commands: &str) -> Commands {
    Commands::new(commands)
}
