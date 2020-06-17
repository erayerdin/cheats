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

//! cheats is a shell backend for games. Basically, it helps you invoke code with
//! a provided string line.
//!
//! The library is not yet production-ready. It has a very simple implementation of
//! developer console and might lack some features you might desire.
//!
//! # Shell Grammar
//!
//! There are many conventions about how to cheat. Grand Theft Auto series receive
//! sequential keypresses and invokes functionality. Age of Empires II has a simple
//! textbox to invoke a cheat but its cheats do not have any arguments.
//!
//! In this library, cheats, the developer console of Valve games such as Half-Life,
//! Counter-Strike, Portal, Left 4 Dead, has been an inspiration and it is implemented in
//! such a way.
//!
//!     // this is a comment
//!     # this is a comment as well
//!     cl_hello // without arg
//!     cl_hello Eray # with arg
//!
//! # How to Use
//!
//! First, you need to initialize a [Shell](struct.Shell.html) instance.
//!
//! ```rust
//! use cheats::Shell;
//!
//! let mut shell = Shell::new();
//! ```
//!
//! ## Registering/Unregistering A Code
//!
//! In order to register a cheat code, first, you need to define what it will do
//! upon invoking. There is a [Invokable](code/trait.Invokable.html) trait that
//! you can implement to do that.
//!
//! ```rust
//! use std::io::Write;
//! use cheats::code::Invokable;
//!
//! struct ClHello; // An empty struct to implement.
//!
//! impl Invokable for ClHello {
//!     fn invoke(
//!         &self,
//!         args: &str, // args if given, can be an empty string
//!         mut stdout: Box<&mut dyn Write>, // stdout to write
//!         mut stderr: Box<&mut dyn Write>, // stderr to write
//!     ) {
//!         match args.is_empty() { // is `args` empty?
//!             true => { // if so...
//!                 // write to `stderr` to inform that args were empty
//!                 // you do not have to do that, it is given for demonstration purposes
//!                 stderr
//!                     .write(b"Args are empty.")
//!                     .expect("Could not write to stderr.");
//!                 // since no args were given, write "Hello, world!" to `stdout`
//!                 stdout
//!                     .write(b"Hello, world!")
//!                     .expect("Could not write to stdout.");
//!             }
//!             false => { // if not...
//!                 // build a message saying the name
//!                 let msg: String = format!("Hello, {}!", args);
//!                 // write `msg` to `stdout`
//!                 stdout
//!                     .write(msg.as_bytes()) // mind `as_bytes`
//!                     .expect("Could not write to stdout.");
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Mind [Invokable](code/trait.Invokable.html) receives `args` as plain `&str`. Parsing of
//! arguments is not handled by this library. Also, no args will result in an empty `&str`.
//!
//! Now that you have an [Invokable](code/trait.Invokable.html), you can register it:
//!
//! ```rust
//! // we need to box our Invokable struct
//! // that's because `register` method on `Shell` requires `Box<Invokable>`
//! let invokable = Box::new(ClHello);
//! shell.register("cl_hello", invokable).expect("Could not register the code.");
//! ```
//!
//! `register` method returns [ShellError](enum.ShellError.html) in case:
//!
//!  - [ShellError::CodeAlreadyExists](enum.ShellError.html): the code with same name is
//! already registered, in this case, `"cl_hello"`
//!  - [ShellError::CodeError](enum.ShellError.html): the initialization of code fails
//! due to having an invalid name
//!
//! You can also unregister an existing code:
//!
//! ```rust
//! shell.unregister("cl_hello").expect("Could not unregister the code.");
//! ```
//!
//! `unregister` method returns [ShellError::CodeDoesNotExist](enum.ShellError.html) if,
//! well, the code with given name is not registered before.
//!
//! ## Running Script
//!
//! You can run a cheat code line by doing:
//!
//! ```rust
//! shell.run("cl_hello").expect("Could not run the code.");
//! shell.run("cl_hello Eray").expect("Could not run the code.");
//! ```
//!
//! Running a single line is cool but consider loading a script in runtime. You can
//! pass a file content to `run` method. An example:
//!
//! ```rust
//! use std::fs;
//!
//! // read the file
//! // it does not have to have .script extension, this is just an example
//! let content: String = fs::read_to_string("path/to/file.script")
//!     .expect("Could not read the file.");
//! // convert from String to &str
//! let content_str: &content[..];
//! // run
//! shell.run(content_str).expect("Could not run the code.");
//! ```
//!
//! # Reading Output
//!
//! Of course, a shell is nothing without output. [Shell](struct.Shell.html) has two
//! attributes:
//!
//!  - **stdout:** Standard output.
//!  - **stderr:** Standard output for errors.
//!
//! These attributes are actually a custom [ReadWrite](trait.ReadWrite.html) trait objects,
//! which means you can `read` from or `write` to them.
//!
//! You usually would like to write to these channels while in [Invokable](code/trait.Invokable.html)
//! trait's `invoke` method because these are referenced in there so that you can use them.
//!
//! You can read from `stdout` or `stderr` as below:
//!
//! ```rust
//! // you can do the same with `stderr`
//! let output: String = {
//!     let ref mut stdout = shell.stdout; // take a reference to stdout
//!     let mut stdout_bytes: Vec<u8> = vec![]; // create a vector buffer for bytes
//!     stdout
//!         .read_to_end(&mut stdout_bytes) // read until the end
//!         .expect("Could not read stdout.");
//!     String::from_iter(stdout_bytes.into_iter().map(|b| b as char)) // map u8 bytes to char
//! };
//! ```

use crate::code::Code;
use crate::code::CodeError as CError;
use crate::code::Invokable;
use io::Stream;
use logos::Logos;
use parser::Token;
use snafu::Snafu;
use std::collections::HashSet;
use std::io::{Read, Write};

pub mod code;
pub mod io;
mod parser;

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

    /// Invokes commands with given input. You can read from a file.
    /// The unregistered codes are simply passed.
    pub fn run(&mut self, input: &'a str) -> ShellResult<()> {
        // TODO ShellResult is not needed here, plan a new strategy
        let lex = Token::lexer(input);

        for token in lex {
            match token {
                Token::Code((name, args)) => match self.codes.iter().find(|c| c.name == name) {
                    Some(c) => {
                        c.invokable.invoke(
                            &args[..],
                            Box::new(&mut self.stdout as &mut dyn Write),
                            Box::new(&mut self.stderr as &mut dyn Write),
                        );
                        continue;
                    }
                    None => continue, // TODO plan a better strategy
                },
                _ => continue,
            }
        }

        Ok(())
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
        input,
        case("cl_hello"),
        case("cl_hello Eray"),
        case("cl_whatever"),
        case(""),
        case("\ncl_lorem what")
    )]
    fn run<'a>(mut shell: Shell<'a>, input: &'a str) {
        assert!(shell.run(input).is_ok());
    }

    #[rstest(input, case("cl_hello"), case("cl_hello Eray"))]
    fn run_out<'a>(mut shell: Shell<'a>, input: &'a str) {
        shell.run(input).expect("Could not run input.");
        let stdout = {
            let ref mut stdout = shell.stdout;
            let mut stdout_bytes: Vec<u8> = vec![];
            stdout
                .read_to_end(&mut stdout_bytes)
                .expect("Could not read stdout.");
            String::from_iter(stdout_bytes.into_iter().map(|b| b as char))
        };

        match input.contains("Eray") {
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

    #[rstest]
    fn run_script_file<'a>(mut shell: Shell<'a>) {
        let script = include_str!("../resources/test/example_script_2.txt");
        shell.run(script).expect("Could not run input.");

        let (stdout, stderr) = (
            {
                let ref mut stdout = shell.stdout;
                let mut stdout_bytes: Vec<u8> = vec![];
                stdout
                    .read_to_end(&mut stdout_bytes)
                    .expect("Could not read stdout.");
                String::from_iter(stdout_bytes.into_iter().map(|b| b as char))
            },
            {
                let ref mut stderr = shell.stderr;
                let mut stderr_bytes: Vec<u8> = vec![];
                stderr
                    .read_to_end(&mut stderr_bytes)
                    .expect("Could not read stdout.");
                String::from_iter(stderr_bytes.into_iter().map(|b| b as char))
            },
        );

        assert_eq!(stdout, "Hello, world!Hello, Eray!");
        assert_eq!(stderr, "Args are empty.");
    }
}
