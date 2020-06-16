// Copyright 2020 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::code::Code;
use crate::code::CodeError as CError;
use crate::code::Invokable;
use io::Stream;
use snafu::Snafu;
use std::collections::HashSet;
use std::io::{Read, Write};

pub mod code;
pub mod io;

#[derive(Debug, Snafu)]
pub enum ShellError<'a> {
    #[snafu(display("Code already exists: {}", name))]
    /// This error is returned when a code already exists in the database.
    /// This usually happens when you register a code again.
    CodeAlreadyExists { name: &'a str },
    #[snafu(display("Code does not exist: {}", name))]
    /// This error is returned when a code does not exist in the database.
    /// This usually happens when you try to unregister a code that does not exist.
    CodeDoesNotExist { name: &'a str },
    #[snafu(display("An error occured in Code. {}", err))]
    /// This error is returned when an error occurs in [Code](code/struct.Code.html).
    CodeError { err: CError<'a> },
}

pub type ShellResult<'a, T> = Result<T, ShellError<'a>>;

// ref: https://stackoverflow.com/a/26983395/2926992
pub trait ReadWrite: Read + Write {}
impl<T> ReadWrite for T where T: Read + Write {}

/// A shell for a game.
pub struct Shell<'a> {
    codes: HashSet<Code<'a>>,
    pub stdout: Box<dyn ReadWrite>,
    pub stderr: Box<dyn ReadWrite>,
}

impl<'a> Shell<'a> {
    pub fn new() -> Self {
        Self {
            codes: HashSet::new(),
            stdout: Box::new(Stream::new()),
            stderr: Box::new(Stream::new()),
        }
    }

    /// Initializes a Shell with custom stream.
    /// By stream, it is meant a struct that implements both [Read][read_trait]
    /// and [Write][write_trait] trait.
    ///
    /// [read_trait]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [write_trait]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn new_with_streams(
        stdout: Option<Box<dyn ReadWrite>>,
        stderr: Option<Box<dyn ReadWrite>>,
    ) -> Self {
        Self {
            codes: HashSet::new(),
            stdout: stdout.unwrap_or(Box::new(Stream::new())),
            stderr: stderr.unwrap_or(Box::new(Stream::new())),
        }
    }

    /// Registers a code to Shell. Returns [CodeAlreadyExists](enum.ShellError.html) if
    /// the code with provided name already exists in the shell.
    pub fn register(&mut self, name: &'a str, invokable: Box<dyn Invokable>) -> ShellResult<()> {
        match self.codes.iter().any(|c| c.name == name) {
            true => Err(ShellError::CodeAlreadyExists { name }),
            false => match Code::new(name, invokable) {
                Ok(c) => {
                    self.codes.insert(c);
                    Ok(())
                }
                Err(e) => Err(ShellError::CodeError { err: e }),
            },
        }
    }

    /// Unregisters a code from Shell. Returns [CodeDoesNotExist](enum.ShellError.html) if
    /// the code with provided name does not exist in the shell.
    pub fn unregister(&mut self, name: &'a str) -> ShellResult<()> {
        if !self.codes.iter().any(|c| c.name == name) {
            return Err(ShellError::CodeAlreadyExists { name });
        }

        self.codes.retain(|c| !(c.name != name));
        Ok(())
    }

    /// Invokes a command with provided line. Line consists of either
    /// `<COMMAND>` or `<COMMAND> <ARGS>`.
    pub fn run(&mut self, line: &'a str) -> ShellResult<()> {
        match line.find(" ") {
            Some(i) => {
                let (name, args) = (&line[0..i], &line[i + 1..line.len()]);

                match self.codes.iter().find(|c| c.name == name) {
                    Some(c) => {
                        c.invokable.invoke(
                            args,
                            Box::new(&mut self.stdout as &mut dyn Write),
                            Box::new(&mut self.stderr as &mut dyn Write),
                        );

                        Ok(())
                    }
                    None => Err(ShellError::CodeDoesNotExist { name }),
                }
            }
            None => match self.codes.iter().find(|c| c.name == line) {
                Some(c) => {
                    c.invokable.invoke(
                        "",
                        Box::new(&mut self.stdout as &mut dyn Write),
                        Box::new(&mut self.stderr as &mut dyn Write),
                    );

                    Ok(())
                }
                None => Err(ShellError::CodeDoesNotExist { name: line }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::iter::FromIterator;

    struct ClHello;
    impl Invokable for ClHello {
        fn invoke(
            &self,
            args: &str,
            mut stdout: Box<&mut dyn Write>,
            mut stderr: Box<&mut dyn Write>,
        ) {
            match args.is_empty() {
                true => {
                    stderr
                        .write(b"Args are empty.")
                        .expect("Could not write to stderr.");
                    stdout
                        .write(b"Hello, world!")
                        .expect("Could not write to stdout.");
                }
                false => {
                    let msg: String = format!("Hello, {}!", args);
                    stdout
                        .write(msg.as_bytes())
                        .expect("Could not write to stdout.");
                }
            }
        }
    }

    #[fixture]
    fn invokable() -> Box<dyn Invokable> {
        Box::new(ClHello)
    }

    #[fixture]
    fn shell<'a>(invokable: Box<dyn Invokable>) -> Shell<'a> {
        let mut shell = Shell::new();
        shell
            .codes
            .insert(Code::new("cl_hello", invokable).expect("Could not initialize Code ch_hello."));
        shell
    }

    #[rstest(
        name,
        expect_failure,
        case("cl_foo", false),
        case("cl_hello", true),
        case("foo bar", true)
    )]
    fn register<'a>(
        mut shell: Shell<'a>,
        invokable: Box<dyn Invokable>,
        name: &'a str,
        expect_failure: bool,
    ) {
        match expect_failure {
            true => {
                assert!(shell.register(name, invokable).is_err());
            }
            false => {
                assert!(shell.register(name, invokable).is_ok());
            }
        }
    }

    #[rstest(name, expect_failure, case("cl_foo", true), case("cl_hello", false))]
    fn unregister<'a>(mut shell: Shell<'a>, name: &'a str, expect_failure: bool) {
        match expect_failure {
            true => assert!(shell.unregister(name).is_err()),
            false => assert!(shell.unregister(name).is_ok()),
        }
    }

    #[rstest(
        line,
        expect_failure,
        case("cl_hello", false),
        case("cl_hello Eray", false),
        case("cl_whatever", true),
        case("", true),
        case("\ncl_lorem what", true)
    )]
    fn run<'a>(mut shell: Shell<'a>, line: &'a str, expect_failure: bool) {
        match expect_failure {
            true => assert!(shell.run(line).is_err()),
            false => assert!(shell.run(line).is_ok()),
        }
    }

    #[rstest(line, case("cl_hello"), case("cl_hello Eray"))]
    fn run_out<'a>(mut shell: Shell<'a>, line: &'a str) {
        shell.run(line).expect("Could not run line.");
        let stdout = {
            let ref mut stdout = shell.stdout;
            let mut stdout_bytes: Vec<u8> = vec![];
            stdout
                .read_to_end(&mut stdout_bytes)
                .expect("Could not read stdout.");
            String::from_iter(stdout_bytes.into_iter().map(|b| b as char))
        };

        match line.contains("Eray") {
            true => {
                assert_eq!(stdout, "Hello, Eray!");
            }
            false => {
                let stderr = {
                    let ref mut stderr = shell.stderr;
                    let mut stderr_bytes: Vec<u8> = vec![];
                    stderr
                        .read_to_end(&mut stderr_bytes)
                        .expect("Could not read stdout.");
                    String::from_iter(stderr_bytes.into_iter().map(|b| b as char))
                };

                assert_eq!(stdout, "Hello, world!");
                assert_eq!(stderr, "Args are empty.");
            }
        }
    }
}
