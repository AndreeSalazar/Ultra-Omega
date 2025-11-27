// Parser de expresiones tipo Houdini
// Soporta: ch("nodo/param"), ch("nodo"), variables, operaciones básicas

#[allow(dead_code)] // Listo para usar
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    Ch(String),           // ch("path")
    Variable(String),     // $variable
    Number(f64),
    String(String),
    Operator(String),     // +, -, *, /, ==, !=, <, >, <=, >=
    ParenOpen,
    ParenClose,
    Comma,
}

#[allow(dead_code)] // Listo para usar
#[derive(Debug, Clone)]
pub struct Expression {
    pub tokens: Vec<ExpressionToken>,
    pub source: String,
}

#[allow(dead_code)] // Listo para usar
pub struct ExpressionParser;

impl ExpressionParser {
    /// Parsear una expresión
    pub fn parse(expr: &str) -> Result<Expression, String> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                ' ' | '\t' | '\n' => continue, // Ignorar whitespace
                
                '(' => tokens.push(ExpressionToken::ParenOpen),
                ')' => tokens.push(ExpressionToken::ParenClose),
                ',' => tokens.push(ExpressionToken::Comma),
                
                '+' => tokens.push(ExpressionToken::Operator("+".to_string())),
                '-' => {
                    // Puede ser número negativo o operador
                    if let Some(&next) = chars.peek() {
                        if next.is_ascii_digit() || next == '.' {
                            // Es un número negativo
                            let num_str = Self::parse_number(&mut chars, true);
                            if let Ok(num) = num_str.parse::<f64>() {
                                tokens.push(ExpressionToken::Number(-num));
                            }
                        } else {
                            tokens.push(ExpressionToken::Operator("-".to_string()));
                        }
                    } else {
                        tokens.push(ExpressionToken::Operator("-".to_string()));
                    }
                },
                '*' => tokens.push(ExpressionToken::Operator("*".to_string())),
                '/' => tokens.push(ExpressionToken::Operator("/".to_string())),
                '=' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(ExpressionToken::Operator("==".to_string()));
                    } else {
                        return Err("Unexpected '=' (use '==' for comparison)".to_string());
                    }
                },
                '!' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(ExpressionToken::Operator("!=".to_string()));
                    } else {
                        tokens.push(ExpressionToken::Operator("!".to_string()));
                    }
                },
                '<' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(ExpressionToken::Operator("<=".to_string()));
                    } else {
                        tokens.push(ExpressionToken::Operator("<".to_string()));
                    }
                },
                '>' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(ExpressionToken::Operator(">=".to_string()));
                    } else {
                        tokens.push(ExpressionToken::Operator(">".to_string()));
                    }
                },
                
                '"' | '\'' => {
                    // String literal
                    let quote = ch;
                    let mut string = String::new();
                    while let Some(c) = chars.next() {
                        if c == quote {
                            break;
                        } else if c == '\\' {
                            // Escape sequence
                            if let Some(escaped) = chars.next() {
                                match escaped {
                                    'n' => string.push('\n'),
                                    't' => string.push('\t'),
                                    '\\' => string.push('\\'),
                                    _ => string.push(escaped),
                                }
                            }
                        } else {
                            string.push(c);
                        }
                    }
                    tokens.push(ExpressionToken::String(string));
                },
                
                '$' => {
                    // Variable: $variable
                    let mut var_name = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(ExpressionToken::Variable(var_name));
                },
                
                'c' | 'C' => {
                    // Posible ch()
                    let mut ident = String::from(ch);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if ident.to_lowercase() == "ch" {
                        // Parsear ch("path")
                        if chars.peek() == Some(&'(') {
                            chars.next(); // Consumir '('
                            
                            // Esperar string
                            if let Some(&quote) = chars.peek() {
                                if quote == '"' || quote == '\'' {
                                    chars.next(); // Consumir quote
                                    let mut path = String::new();
                                    while let Some(c) = chars.next() {
                                        if c == quote {
                                            break;
                                        } else if c == '\\' {
                                            if let Some(escaped) = chars.next() {
                                                path.push(escaped);
                                            }
                                        } else {
                                            path.push(c);
                                        }
                                    }
                                    
                                    tokens.push(ExpressionToken::Ch(path));
                                    
                                    // Esperar ')'
                                    if chars.peek() == Some(&')') {
                                        chars.next();
                                    } else {
                                        return Err("Expected ')' after ch()".to_string());
                                    }
                                } else {
                                    return Err("Expected string in ch()".to_string());
                                }
                            } else {
                                return Err("Expected string in ch()".to_string());
                            }
                        } else {
                            // No es ch(), es una variable que empieza con 'c'
                            tokens.push(ExpressionToken::Variable(ident));
                        }
                    } else {
                        tokens.push(ExpressionToken::Variable(ident));
                    }
                },
                
                '0'..='9' | '.' => {
                    // Número
                    let num_str = Self::parse_number(&mut chars, false);
                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(ExpressionToken::Number(num));
                    } else {
                        return Err(format!("Invalid number: {}", num_str));
                    }
                },
                
                _ if ch.is_alphabetic() || ch == '_' => {
                    // Identificador (variable)
                    let mut ident = String::from(ch);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(ExpressionToken::Variable(ident));
                },
                
                _ => return Err(format!("Unexpected character: {}", ch)),
            }
        }
        
        Ok(Expression {
            tokens,
            source: expr.to_string(),
        })
    }
    
    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>, is_negative: bool) -> String {
        let mut num_str = String::new();
        if is_negative {
            num_str.push('-');
        }
        
        let mut has_dot = false;
        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    num_str.push(chars.next().unwrap());
                },
                '.' if !has_dot => {
                    has_dot = true;
                    num_str.push(chars.next().unwrap());
                },
                _ => break,
            }
        }
        
        num_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ch() {
        let expr = ExpressionParser::parse(r#"ch("nodo/codigo")"#).unwrap();
        assert_eq!(expr.tokens.len(), 1);
        if let ExpressionToken::Ch(path) = &expr.tokens[0] {
            assert_eq!(path, "nodo/codigo");
        } else {
            panic!("Expected Ch token");
        }
    }
    
    #[test]
    fn test_parse_expression() {
        let expr = ExpressionParser::parse(r#"ch("nodo1") + ch("nodo2")"#).unwrap();
        assert!(expr.tokens.len() >= 3);
    }
}

