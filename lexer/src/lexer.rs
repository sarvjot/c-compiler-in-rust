use regex::Regex;
use crate::tokens::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    regexes: Vec<(TokenType, Regex)>,
    whitespace_regex: Regex,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let regexes: Vec<(TokenType, Regex)> = vec![
            // Word boundary '\b' prevents partial matches
            (TokenType::Identifier, Regex::new(r"^([a-zA-Z_]\w*)\b").unwrap()),
            (TokenType::Constant, Regex::new(r"^([0-9]+)\b").unwrap()),
            (TokenType::IntKeyword, Regex::new(r"^(int)\b").unwrap()),
            (TokenType::VoidKeyword, Regex::new(r"^(void)\b").unwrap()),
            (TokenType::ReturnKeyword, Regex::new(r"^(return)\b").unwrap()),
            (TokenType::OpenParen, Regex::new(r"^(\()").unwrap()),
            (TokenType::CloseParen, Regex::new(r"^(\))").unwrap()),
            (TokenType::OpenBrace, Regex::new(r"^(\{)").unwrap()),
            (TokenType::CloseBrace, Regex::new(r"^(\})").unwrap()),
            (TokenType::Semicolon, Regex::new(r"^(;)").unwrap()),
        ];

        Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
            regexes,
            whitespace_regex: Regex::new(r"^\s+").unwrap(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, String> {
        while self.position < self.input.len() {
            let current_line = self.line;
            let current_column = self.column;

            let remaining_input = &self.input[self.position..];

            if let Some(mat) = self.whitespace_regex.find(remaining_input) {
                let lexeme = mat.as_str().to_string();
                self.advance_position(&lexeme);
                continue;
            }

            let mut longest_match_data: Option<(TokenType, String)> = None;
            let mut longest_match_len = 0;

            for (token_type, regex) in &self.regexes {
                if let Some(mat) = regex.find(remaining_input) {
                    let lexeme_str = mat.as_str();
                    if lexeme_str.len() > longest_match_len {
                        longest_match_len = lexeme_str.len();
                        longest_match_data = Some((token_type.clone(), lexeme_str.to_string()));
                    }
                }
            }

            if let Some((token_type, lexeme_owned)) = longest_match_data {
                let token = Token {
                    token_type,
                    lexeme: lexeme_owned,
                    line: current_line,
                    column: current_column,
                };
                self.advance_position(&token.lexeme);
                return Ok(Some(token));
            } else {
                return Err(format!(
                    "Lexical error at line {}, column {}: Unexpected character '{}'",
                    current_line,
                    current_column,
                    remaining_input.chars().next().unwrap_or(' ')
                ));
            }
        }
        Ok(None)
    }

    fn advance_position(&mut self, lexeme: &str) {
        for char_code_point in lexeme.chars() {
            self.position += char_code_point.len_utf8();
            if char_code_point == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}