#[macro_use]
extern crate cached;

use regex::RegexBuilder;
use std::error::Error;
use std::io::{copy, stdin, stdout, Result as IoResult, Write};
use std::process::{Command, Stdio};
use subprocess::{Exec, NullFile};

mod compgen;
use self::compgen::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = stdin();
    let stdout = stdout();

    let mut memory = Vec::new();

    {
        let mut writer = MultiWriter {
            writers: vec![Box::new(stdout), Box::new(&mut memory)],
        };

        copy(&mut reader, &mut writer)?;
    }

    let output = String::from_utf8(memory)?;

    let command_regex = RegexBuilder::new(r"^\s*((.*\\\n)*.*)$")
        .multi_line(true)
        .build()?;

    command_regex
        .captures_iter(&output)
        .map(|c| c[1].to_string())
        .filter(|c| runnable_command(&c))
        .map(|c| Exec::shell(c).stdout(NullFile).stderr(NullFile))
        .map(|shell| shell.join())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

struct MultiWriter<'w> {
    writers: Vec<Box<dyn Write + 'w>>,
}

impl<'w> Write for MultiWriter<'w> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        for writer in &mut self.writers {
            writer.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}

fn runnable_command(cmd: &str) -> bool {
    let first = cmd.split_whitespace().next();
    let first = match first {
        None => return false,
        Some(f) => f,
    };
    runnable_first(first.to_string())
}

cached! {
    COMMANDS;
    fn runnable_first(first: String) -> bool = {
        Command::new("which")
            .arg(first)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to run which")
            .success()
    }
}
