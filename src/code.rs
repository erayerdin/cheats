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

use derivative::Derivative;
use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
pub enum CodeError<'a> {
    #[snafu(display(
        "Code could not be initialized due to its name containing whitespace. Name: {}",
        name
    ))]
    /// Error if a code name contains any whitespace.
    WhitespaceError { name: &'a str },
}

type CodeResult<'a, T> = Result<T, CodeError<'a>>;

/// A trait for structs that are invokable/runnable.
pub trait Invokable {
    /// Invocation of a series of events.
    ///
    /// `args` can be an empty string if nothing is given. You can use `stdout` and `stderr`
    /// to write output.
    fn invoke(&self, args: &str, stdout: Box<&mut dyn io::Write>, stderr: Box<&mut dyn io::Write>);
}

#[derive(Derivative)]
#[derivative(Eq, PartialEq, Hash)]
/// A cheat code.
pub(crate) struct Code<'a> {
    pub name: &'a str,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    pub invokable: Box<dyn Invokable>,
}

impl<'a> Code<'a> {
    pub(crate) fn new(name: &'a str, invokable: Box<dyn Invokable>) -> CodeResult<Self> {
        match name.chars().any(|c| c.is_whitespace()) {
            true => Err(CodeError::WhitespaceError { name }),
            false => Ok(Self { name, invokable }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    struct ClHello;
    impl Invokable for ClHello {
        fn invoke(
            &self,
            _args: &str,
            _stdout: Box<&mut dyn std::io::Write>,
            _stderr: Box<&mut dyn std::io::Write>,
        ) {
            todo!()
        }
    }

    #[fixture]
    fn invokable() -> Box<dyn Invokable> {
        Box::new(ClHello)
    }

    #[rstest]
    fn code_new_success(invokable: Box<dyn Invokable>) {
        let code_r = Code::new("cl_hello", invokable);
        assert!(code_r.is_ok());
    }
}
