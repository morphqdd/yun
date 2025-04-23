pub struct Shell {
    current_command: String,
}

impl Shell {
    pub fn new() -> Self {
        Self { current_command: "".to_string() }
    }

    pub fn get_command(&self) -> &str {
        &self.current_command
    }

    pub fn set_command(&mut self, command: String) {
        self.current_command = command;
    }
}

impl AsMut<Shell> for Shell {
    fn as_mut(&mut self) -> &mut Shell {
        &mut self
    }
}