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

use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
enum CodeError<'a> {
    #[snafu(display(
        "Code could not be initialized due to its name containing whitespace. Name: {}",
        name
    ))]
    /// Error if a code name contains any whitespace.
    WhitespaceError { name: &'a str },
}

type CodeResult<'a, T> = Result<T, CodeError<'a>>;

/// A trait for structs that are invokable/runnable.
trait Invokable {
    fn invoke(&self, args: &str, stdout: Box<dyn io::Write>, stderr: Box<dyn io::Write>);
}

struct Code<'a> {
    name: &'a str,
    invokable: Box<dyn Invokable>,
}

impl<'a> Code<'a> {
    fn new(name: &'a str, invokable: Box<dyn Invokable>) -> CodeResult<Self> {
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
            args: &str,
            stdout: Box<dyn std::io::Write>,
            stderr: Box<dyn std::io::Write>,
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
