/// Serialization trait.
/// 
/// 
/// Types which implement `Jsonable` can be converted into a valid json string.
pub trait Jsonable {

    /// Converts the struct into a valid json string.
    fn into_json(&self) -> String;


    /// Converts a valid json string into the given type.
    fn from_json(json_string: &str) -> Result<Self, Box<dyn std::error::Error>>
    where Self:Sized;
}





#[derive(Debug,PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String,JsonValue)>),
}



#[derive(Clone,Debug,PartialEq)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub trait FromJsonValue: Sized {
    fn from_json_value(value: &JsonValue) -> Result<Self,String>;
}



impl FromJsonValue for String {
    fn from_json_value(value: &JsonValue) -> Result<Self,String> {
        if let JsonValue::String(s) = value {
            Ok(s.clone())
        } else {
            Err(format!("Expected string, found {:?}",value))
        }
    }
}



impl FromJsonValue for f64 {
    fn from_json_value(value: &JsonValue) -> Result<Self,String> {
        if let JsonValue::Number(n) = value {
            Ok(*n)
        } else {
            Err(format!("Expected number, found {:?}",value))
        }
    }
}


impl FromJsonValue for bool {
    fn from_json_value(value: &JsonValue) -> Result<Self,String> {
        if let JsonValue::Boolean(b) = value {
            Ok(*b)
        } else {
            Err(format!("Expected boolean, found {:?}",value))
        }
    }
}


impl<T:FromJsonValue> FromJsonValue for Option<T> {
    fn from_json_value(value: &JsonValue) -> Result<Self,String> {
        match value {
            JsonValue::Null => Ok(None),
            _ => Ok(Some(T::from_json_value(value)?)),
        }
    }
}

impl<T:FromJsonValue> FromJsonValue for Vec<T> {
    fn from_json_value(value: &JsonValue) -> Result<Self,String> {
        if let JsonValue::Array(arr) = value {
            let mut result = Vec::with_capacity(arr.len());
            for item in arr {
                result.push(T::from_json_value(item)?);
            }
            Ok(result)
        } else {
            Err(format!("Expected array, found {:?}",value))
        }
    }
}







fn tokenize(input: &str) -> Result<Vec<Token>,String> {
    let mut tokens = vec![];

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '{' => {tokens.push(Token::CurlyOpen); chars.next(); },
            '}' => {tokens.push(Token::CurlyClose); chars.next(); },
            '[' => {tokens.push(Token::BracketOpen); chars.next(); },
            ']' => {tokens.push(Token::BracketClose); chars.next(); },
            ':' => {tokens.push(Token::Colon); chars.next(); },
            ',' => {tokens.push(Token::Comma); chars.next(); },
            '"' => {
                chars.next(); // consume `"`
                let mut s = String::new();

                while let Some(ch) = chars.next() {
                    if ch == '"' {break;}
                    s.push(ch);
                }
                tokens.push(Token::String(s));
            },
            '0'..='9' | '-' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                        num_str.push(chars.next().unwrap());
                    } else  {
                        break;
                    }
                } 
                tokens.push(Token::Number(num_str.parse().map_err(|_| "Invalid number.".to_string())?));

            },
            't' => {
                if chars.by_ref().take(4).collect::<String>() == "true".to_string() {
                    tokens.push(Token::Boolean(true));
                } else {
                    return Err("Invalid token".to_string());
                }
            },
            'f' => {
                if chars.by_ref().take(5).collect::<String>() == "false".to_string() {
                    tokens.push(Token::Boolean(false));
                } else {
                    return Err("Invalid token".to_string());
                }
            },
            'n' => {
                if chars.by_ref().take(4).collect::<String>() == "null".to_string() {
                    tokens.push(Token::Null);
                } else {
                    return Err("Invalid token".to_string());
                }
            },
            c if c.is_whitespace() => {chars.next();},
            _ => {
                return Err(format!("Unexpected character: {c}"));
            }
        }
    }
    Ok(tokens)
}



pub struct Parser {
    tokens:Vec<Token>,
    position:usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn peek(&self) -> Result<&Token, String> {
        if self.position >= self.tokens.len() {
            Err("Unexpected end of input".to_string())
        } else {
            Ok(&self.tokens[self.position])
        }
    }

    pub fn consume(&mut self) -> Result<Token, String> {
        if self.position >= self.tokens.len() {
            Err("Unexpected end of input".to_string())
        } else {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Ok(token)
        }
    }

    pub fn parse_value(&mut self) -> Result<JsonValue, String> {
        let token = self.peek()?;
        match token {
            Token::Null => {
                self.consume()?;
                Ok(JsonValue::Null)
            },
            Token::Boolean(b) => {
                let val = *b;
                self.consume()?;
                Ok(JsonValue::Boolean(val))
            },
            Token::Number(n) => {
                let val = *n;
                self.consume()?;
                Ok(JsonValue::Number(val))
            },
            Token::String(s) => {
                let val = s.clone();
                self.consume()?;
                Ok(JsonValue::String(val))
            },
            Token::BracketOpen => self.parse_array(),
            Token::CurlyOpen => self.parse_object(),
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    pub fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.consume()?; // Consume '['
        let mut elements = vec![];

        // Check if array is empty
        if self.peek()? == &Token::BracketClose {
            self.consume()?; // Consume ']'
            return Ok(JsonValue::Array(elements));
        }

        loop {
            elements.push(self.parse_value()?);
            
            let next_token = self.peek()?;
            if next_token == &Token::Comma {
                self.consume()?; // Consume ','
            } else if next_token == &Token::BracketClose {
                break;
            } else {
                return Err(format!("Expected comma or closing bracket, got: {:?}", next_token));
            }
        }
        
        self.consume()?; // Consume ']'
        Ok(JsonValue::Array(elements))
    }

    pub fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.consume()?; // Consume '{'

        let mut members = vec![];

        // Check if object is empty
        if self.peek()? == &Token::CurlyClose {
            self.consume()?; // Consume '}'
            return Ok(JsonValue::Object(members));
        }

        loop {
            let key_token = self.consume()?;
            let key = if let Token::String(s) = key_token {
                s
            } else {
                return Err("Expected string key in object.".to_string());
            };
            
            if self.peek()? != &Token::Colon {
                return Err("Expected colon after key.".to_string());
            }
            self.consume()?; // Consume ':'
            
            let value = self.parse_value()?;
            members.push((key, value));

            let next_token = self.peek()?;
            if next_token == &Token::Comma {
                self.consume()?; // Consume ','
            } else if next_token == &Token::CurlyClose {
                break;
            } else {
                return Err(format!("Expected comma or closing curly brace, got: {:?}", next_token));
            }
        }
        
        self.consume()?; // Consume '}'
        Ok(JsonValue::Object(members))
    }

    pub fn parse_json(input: &str) -> Result<JsonValue, String> {
        let tokens = tokenize(input)?;
        
        if tokens.is_empty() {
            return Err("Empty input".to_string());
        }

        let mut parser = Parser::new(tokens);
        let result = parser.parse_value()?;
        
        // Check if there are remaining tokens (should be at end)
        if parser.position < parser.tokens.len() {
            return Err(format!("Unexpected tokens after JSON value: {:?}", 
                             &parser.tokens[parser.position..]));
        }
        
        Ok(result)
    }
}
