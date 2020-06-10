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

#[derive(Debug, Snafu)]
/// Errors related to shell. Keep in mind that code names and args are not provided
/// in these errors. One needs to provide them as message.
enum ShellError {
    #[snafu(display("An error occured while executing code: {}", message))]
    /// Error while invoking a code.
    InvocationError { message: String },
    #[snafu(display("An error occured while initializing code: {}", message))]
    /// Error while initializing a code.
    CodeError { message: String },
}

type ShellResult<T> = Result<T, ShellError>;
