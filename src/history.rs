
pub mod history {
    pub struct History {
        command: Vec<String>
    }

    impl History {
        pub fn new() -> History {
            return History {
                command: Vec::new()
            }
        }
        pub fn add(&mut self, line: &str) {
            self.command.push(String::from(line));
        }
        pub fn print(&self) {
            for i in 0..self.command.len() {
                println!("{} : {}", i, self.command[i]);
            }
        }
    }
}