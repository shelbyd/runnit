use std::collections::HashSet;
use std::error::Error;
use subprocess::Exec;

pub struct Compgen {
    commands: HashSet<String>,
}

impl Compgen {
    pub fn create() -> Result<Self, Box<dyn Error>> {
        let capture = Exec::shell("compgen -c").capture()?;
        Ok(Compgen {
            commands: capture
                .stdout_str()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        })
    }

    pub fn runnable_command(&self, cmd: &str) -> bool {
        let first = cmd.split_whitespace().next();
        let first = match first {
            None => return false,
            Some(f) => f,
        };
        self.commands.contains(first)
    }
}
