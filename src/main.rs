use regex::RegexBuilder;
use std::error::Error;
use std::io::{copy, stdin, stdout, Result as IoResult, Write};
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

    let compgen = Compgen::create()?;

    command_regex
        .captures_iter(&output)
        .map(|c| c[1].to_string())
        .filter(|c| compgen.runnable_command(&c))
        .map(|c| Exec::shell(c).stdout(NullFile).stderr(NullFile).join())
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
