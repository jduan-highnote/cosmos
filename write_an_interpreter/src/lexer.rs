use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    //    position: usize,      // current position in input
    //    read_position: usize, // next position in input, used for peaking at the next char
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            //            position: 0,
            //            read_position: 1,
            iter: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(ch) = self.iter.next() {
            match ch {
                ',' => Token::Delimiter(Delimiter::Comma),
                '{' => Token::Delimiter(Delimiter::LeftBrace),
                '(' => Token::Delimiter(Delimiter::LeftParen),
                '}' => Token::Delimiter(Delimiter::RightBrace),
                ')' => Token::Delimiter(Delimiter::RightParen),
                ';' => Token::Delimiter(Delimiter::Semicolon),
                '=' => Token::Operator(Operator::Assignment),
                '+' => Token::Operator(Operator::PlusSign),
                '-' => Token::Operator(Operator::MinusSign),
                '!' => Token::Operator(Operator::Bang),
                '*' => Token::Operator(Operator::Asterisk),
                '/' => Token::Operator(Operator::Slash),
                '<' => Token::Operator(Operator::LessThan),
                '>' => Token::Operator(Operator::GreaterThan),
                // TODO: handle other kinds of tokens, such as identifiers, keywords etc.
                n if n.is_alphabetic() => {
                    let identifier = self.read_identifier(ch);
                    match Keyword::convert(&identifier) {
                        Some(keybword) => Token::Keyword(keybword),
                        None => Token::Identifier(identifier),
                    }
                }
                n if n.is_numeric() => Token::Literal(self.read_integer(ch)),
                _ => Token::SpecialToken(SpecialToken::Illegal),
            }
        } else {
            Token::SpecialToken(SpecialToken::EOF)
        }
    }

    fn read_identifier(&mut self, ch: char) -> Identifier {
        let mut buffer = vec![ch];
        while let Some(ch) = self.iter.peek() {
            if Self::is_letter(*ch) {
                buffer.push(*ch);
                self.iter.next();
            } else {
                break;
            }
        }

        buffer.into_iter().collect()
    }

    fn read_integer(&mut self, ch: char) -> Literal {
        let mut buffer = vec![ch];
        while let Some(ch) = self.iter.peek() {
            if ch.is_numeric() {
                buffer.push(*ch);
                self.iter.next();
            } else {
                break;
            }
        }

        let s: String = buffer.into_iter().collect();
        Literal::Integer(s.parse::<i32>().unwrap())
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.iter.peek() {
            if ch.is_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    SpecialToken(SpecialToken),
    Identifier(Identifier),
    Literal(Literal),
    Operator(Operator),
    Delimiter(Delimiter),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpecialToken {
    EOF,
    Illegal,
}

pub type Identifier = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Integer(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Assignment,
    PlusSign,
    MinusSign,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Comma,
    LeftBrace,
    LeftParen,
    RightBrace,
    RightParen,
    Semicolon,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

impl Keyword {
    pub fn convert(identifier: &str) -> Option<Self> {
        match identifier {
            "fn" => Some(Keyword::Function),
            "let" => Some(Keyword::Let),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "return" => Some(Keyword::Return),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_char() {
        let mut lexer = Lexer::new("=+(){},;!");
        assert_eq!(Token::Operator(Operator::Assignment), lexer.next_token());
        assert_eq!(Token::Operator(Operator::PlusSign), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::LeftParen), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::RightParen), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::LeftBrace), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::RightBrace), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::Comma), lexer.next_token());
        assert_eq!(Token::Delimiter(Delimiter::Semicolon), lexer.next_token());
        assert_eq!(
            Token::SpecialToken(SpecialToken::Illegal),
            lexer.next_token()
        );
        assert_eq!(Token::SpecialToken(SpecialToken::EOF), lexer.next_token());
    }

    #[test]
    fn full_test() {
        let input = r"
let five = 5;
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
        return true;
} else {
        return false;
}

10 == 10;
10 != 9;
        ";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier(String::from("five")),
            Token::Operator(Operator::Assignment),
            Token::Literal(Literal::Integer(5)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Keyword(Keyword::Let),
            Token::Identifier(String::from("ten")),
            Token::Operator(Operator::Assignment),
            Token::Literal(Literal::Integer(10)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Keyword(Keyword::Let),
            Token::Identifier(String::from("add")),
            Token::Operator(Operator::Assignment),
            Token::Keyword(Keyword::Function),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Identifier(String::from("x")),
            Token::Delimiter(Delimiter::Comma),
            Token::Identifier(String::from("y")),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Identifier(String::from("x")),
            Token::Operator(Operator::PlusSign),
            Token::Identifier(String::from("y")),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Keyword(Keyword::Let),
            Token::Identifier(String::from("result")),
            Token::Operator(Operator::Assignment),
            Token::Identifier(String::from("add")),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Identifier(String::from("five")),
            Token::Delimiter(Delimiter::Comma),
            Token::Identifier(String::from("ten")),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Operator(Operator::Bang),
            Token::Operator(Operator::MinusSign),
            Token::Operator(Operator::Slash),
            Token::Operator(Operator::Asterisk),
            Token::Literal(Literal::Integer(5)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Literal(Literal::Integer(5)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(10)),
            Token::Operator(Operator::GreaterThan),
            Token::Literal(Literal::Integer(5)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Keyword(Keyword::If),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Literal(Literal::Integer(5)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(10)),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Keyword(Keyword::Return),
            Token::Keyword(Keyword::True),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Keyword(Keyword::Else),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Keyword(Keyword::Return),
            Token::Keyword(Keyword::False),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Literal(Literal::Integer(10)),
        ];

        for expected_token in expected_tokens {
            assert_eq!(expected_token, lexer.next_token());
        }
    }

    #[test]
    fn test_read_identifier() {
        let mut lexer = Lexer::new("hello world");
        assert_eq!(Token::Identifier(String::from("hello")), lexer.next_token());
    }

    #[test]
    fn test_hello() {
        let mut iter = "hello".chars().peekable();
        assert_eq!('h', iter.next().unwrap());
        assert_eq!('e', iter.next().unwrap());
        assert_eq!(10, "10".parse::<i32>().unwrap());
    }
}