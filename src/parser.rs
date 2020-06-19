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

use log::*;
use logos::Lexer;
use logos::Logos;

struct LexerCallbacks;

impl LexerCallbacks {
    fn code(lex: &mut Lexer<Token>) -> (String, String) {
        debug!("Running lexer callback: code");

        let content: &str = lex.slice().trim();
        let mut split = content.split(r" ");

        let (name, args) = (
            split
                .next()
                .expect("Parsing code unexpectedly failed.")
                .to_owned(),
            split.next().unwrap_or("").to_owned(),
        );

        trace!("name: {}", name);
        trace!("args: {}", args);

        (name, args)
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Error,

    #[regex(r#"[a-zA-Z0-9-_]+([ ][a-zA-Z0-9-_]+)*"#, LexerCallbacks::code)]
    Code((String, String)),
    #[regex("[//|#](.*)", logos::skip)]
    Comment,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn lexer() {
        let script: &str = include_str!("../resources/test/example_script.txt");
        let mut lex = Token::lexer(script);

        assert_eq!(
            lex.next(),
            Some(Token::Code(("cl_hello".to_owned(), "".to_owned())))
        );
        assert_eq!(
            lex.next(),
            Some(Token::Code(("cl_hello".to_owned(), "Eray".to_owned())))
        );
        assert_eq!(
            lex.next(),
            Some(Token::Code(("cl_hello".to_owned(), "".to_owned())))
        );
        assert_eq!(
            lex.next(),
            Some(Token::Code(("cl_hello".to_owned(), "Eray".to_owned())))
        );
        assert_eq!(
            lex.next(),
            Some(Token::Code(("cl_hello".to_owned(), "Eray".to_owned())))
        );
    }
}
