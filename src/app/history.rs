#[derive(Default)]
pub struct History {
    history: Vec<String>,
    pointer: usize,
}

impl History {
    pub fn push(&mut self, command: impl Into<String>) {
        self.history.push(command.into());
        self.pointer = 0;
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.pointer >= self.history.len() {
            None
        } else {
            let prev = self.history.get(self.history.len() - 1 - self.pointer);
            self.pointer += 1;
            prev
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.pointer == 0 {
            None
        } else {
            let next = self.history.get(self.history.len() + 1 - self.pointer);
            self.pointer -= 1;
            next
        }
    }
}
