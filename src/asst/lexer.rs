use std::collections::HashMap;
use std::io::Error;
#[derive(Debug, Copy, Clone)]
enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenKind,
    lexeme: String,
    line: usize,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenKind, lexeme: String, line: usize, literal: String) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    //   pub fn to_string(&self) -> String {
    //     let token_str : String = String::new();
    //   token_str.push_str(&self.token_type);
    // token_str.push_str(" ");
    // token_str.push_str(&self.lexeme);
    // return token_str;
    // }
}

pub struct Scanner<'a> {
    source: String,
    token: &'a mut Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
    keywords: HashMap<String, TokenKind>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, token :&'a mut Vec<Token>) -> Self {
        Scanner {
            source,
            token,
            current: 0,
            start: 0,
            line: 1,
            keywords: Scanner::add_keywords(),
        }
    }

    fn add_keywords() -> HashMap<String, TokenKind> {
        let mut keywords: HashMap<String, TokenKind> = HashMap::new();
        keywords.insert("and".to_string(), TokenKind::AND);
        keywords.insert("class".to_string(), TokenKind::CLASS);
        keywords.insert("else".to_string(), TokenKind::ELSE);
        keywords.insert("false".to_string(), TokenKind::FALSE);
        keywords.insert("for".to_string(), TokenKind::FOR);
        keywords.insert("fun".to_string(), TokenKind::FUN);
        keywords.insert("if".to_string(), TokenKind::IF);
        keywords.insert("nil".to_string(), TokenKind::NIL);
        keywords.insert("or".to_string(), TokenKind::OR);
        keywords.insert("print".to_string(), TokenKind::PRINT);
        keywords.insert("return".to_string(), TokenKind::RETURN);
        keywords.insert("super".to_string(), TokenKind::SUPER);
        keywords.insert("this".to_string(), TokenKind::THIS);
        keywords.insert("true".to_string(), TokenKind::TRUE);
        keywords.insert("var".to_string(), TokenKind::VAR);
        keywords.insert("while".to_string(), TokenKind::WHILE);

        keywords
    }
    fn is_at_end(&mut self) -> bool {
        &self.current >= &self.source.chars().count()
    }

    pub fn scan_tokens(&mut self) {
        while !&self.is_at_end() {
            self.start = self.current;
            let _ = &self.scan_token();
        }

        let token : Token = Token {
            token_type : TokenKind::EOF,
            lexeme : "".to_string(),
            line : self.line,
            literal : "".to_string(),
        };
        self.token.push(token);
    }

    fn scan_token(&mut self) -> Result<bool, Error> {
        let c: char = self.advance();
        match c {
            //Single Characters
            '(' => Ok(self.add_token(TokenKind::LEFT_PAREN)),
            ')' => Ok(self.add_token(TokenKind::RIGHT_PAREN)),
            '{' => Ok(self.add_token(TokenKind::LEFT_BRACE)),
            '}' => Ok(self.add_token(TokenKind::RIGHT_BRACE)),
            ',' => Ok(self.add_token(TokenKind::COMMA)),
            '.' => Ok(self.add_token(TokenKind::DOT)),
            '-' => Ok(self.add_token(TokenKind::MINUS)),
            '+' => Ok(self.add_token(TokenKind::PLUS)),
            ';' => Ok(self.add_token(TokenKind::SEMICOLON)),
            '*' => Ok(self.add_token(TokenKind::STAR)),

            //Two Characters
            '!' => match &self.check('=') {
                true => Ok(self.add_token(TokenKind::BANG_EQUAL)),
                false => Ok(self.add_token(TokenKind::BANG)),
            },
            '=' => match &self.check('=') {
                true => Ok(self.add_token(TokenKind::EQUAL_EQUAL)),
                false => Ok(self.add_token(TokenKind::EQUAL)),
            },
            '<' => match &self.check('=') {
                true => Ok(self.add_token(TokenKind::LESS_EQUAL)),
                false => Ok(self.add_token(TokenKind::LESS)),
            },
            '>' => match &self.check('=') {
                true => Ok(self.add_token(TokenKind::GREATER_EQUAL)),
                false => Ok(self.add_token(TokenKind::GREATER)),
            },
            '/' => {
                if self.check('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        let _ = self.advance();
                    }
                    Ok(true)
                } else {
                    Ok(self.add_token(TokenKind::SLASH))
                }
            }
            '\n' => {
                self.line += 1;
                Ok(true)
            }
            '"' => {
                self.string();
                Ok(true)
            }
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                    Ok(true)
                } else if c.is_digit(10) || c == '_' {
                    self.identifier();
                    Ok(true)
                } else {
                    let unknown_char = Error::other("shit");
                    Err(unknown_char)
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        if self.peek() == '\0' {
            return '\0';
        }
        self.current += 1;
        self.source.chars().nth(self.current-1).unwrap()
    }

    fn check(&mut self, value: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.peek() != value {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 > self.source.chars().count() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn add_token(&mut self, token_type: TokenKind) -> bool {
        self.add_tokens(token_type, " ".to_string())
    }

    fn add_tokens(&mut self, token_type: TokenKind, literal: String) -> bool {
        let lexeme = &self.source[self.start..self.current];
        let single_token: Token = Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
            literal,
        };
        self.token.push(single_token);
        return true;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }

        if self.is_at_end() {
            return;
        }

        let _ = self.advance();
        let text = &self.source[self.start..self.current];
        self.add_tokens(TokenKind::STRING, text.to_string());
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            let _ = self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            let _ = self.advance();

            while self.peek().is_ascii_digit() {
                let _ = self.advance();
            }
        }

        self.add_tokens(
            TokenKind::NUMBER,
            self.source[self.start..self.current].to_string(),
            );
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            let _ = self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_type) = self.keywords.get(text) {
            self.add_token(*token_type);
        } else {
            self.add_token(TokenKind::IDENTIFIER);
        }
    }
}
