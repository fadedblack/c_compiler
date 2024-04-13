pub mod lexer;

enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
struct  Token {
    token_type : TokenKind,
    lexeme : &str,
    line : usize,
}


impl Token{
    pub fn new(token_type : TokenKind, lexeme : &str, line : usize) -> self{
        Token (self.token_type = token_type, self.lexeme = lexeme, self.line = line)
    }

    pub fn to_String(&self) -> &str {
        let token_str : &str = self.token_type + " " + self.lexeme;
        return token_str;
    } 
}


#[derive(Debug)]
struct Scanner {
    source : String,
    token : Vec<Token>,
}


impl Scanner {
    // add code here
    pub fn new(source : String, token : Vec<Token>) -> self {
        let mut current : usize = 0;
        let mut start : usize = 0;
        let mut line : usize = 1;

        self (self.source = source, self.token = token)
    }


    pub fn scan_tokens(&self) {
        while !isAtEnd(current) {
            start = current;
             scan_token();
        }  
    }


    fn isAtEnd(&self, current : usize) -> Bool { 
        {current >= &self.source.count()}
    }


    fn scan_token(&self) -> Result<Bool, Err()> {
        let mut c : char = advance();
        match c {
            //Single Characters
            '(' => {Ok(addToken(TokenKind::LEFT_PAREN))}
            ')' => {Ok(addToken (TokenKind::RIGHT_PAREN))}
            '{' => {Ok(addToken (TokenKind::LEFT_BRACE))}
            '}' => {Ok(addToken (TokenKind::RIGHT_BRACE))}
            ',' => {Ok(addToken (TokenKind::COMMA))}
            '.' => {Ok(addToken (TokenKind::DOT))}
            '-' => {Ok(addToken (TokenKind::MINUS))}
            '+' => {Ok(addToken (TokenKind::PLUS))}
            ';' => {Ok(addToken (TokenKind::SEMICOLON))}
            //Two Characters
            '!' => { 
                match check('=') {
                    true => {Ok(addToken (TokenKind::BANG_EQUAL))}
                    false => {Ok(addToken (TokenKind::BANG))}
                }
            }
            '=' => { 
                match check('=') {
                    true => {Ok(addToken (TokenKind::EQUAL_EQUAL))}
                    false => {Ok(addToken (TokenKind::EQUAL))}
                }
            }
            '<' => { 
                match check('=') {
                    true => {Ok(addToken (TokenKind::LESS_EQUAL))}
                    false => {Ok(addToken (TokenKind::LESS))}
                }
            }
            '>' => { 
                match check('=') {
                    true => {Ok(addToken (TokenKind::GREATER_EQUAL))}
                    false => {Ok(addToken (TokenKind::GREATER))}
                }
            }
            '/' => {
                if check('/') {
                    while peek() != '\n' && isAtEnd() {
                        advance();
                    }
                }
                else {
                    Ok(addToken (TokenKind::SLASH))
                }
            }
            '\n' => {
                self.line += 1;
            }



        }
    }

    fn advance() -> char {
        self.current += 1;
        &self.source.chars().nth(self.current)
    }


    fn check(value : char) -> Bool {
        if isAtEnd() {
            false
        }
        else if advance() != value {
            self.current -= 1;
            false
        }
        self.current += 1;
        true
    }


    fn peek(&self) -> char {
        if &self.chars().nth(current) {
            return '\0';
        }
        return &self.chars().nth(current);
    }

    fn addToken(&self,tokenType : TokenKind) -> Bool {
        let lexeme : &str = &self.sources[start..current];
        let token = Token (token_type, lexeme, self.line);
        Token.token.push(token);
        return true;
    }

}

