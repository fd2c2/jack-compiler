use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType { Keyword, Symbol, Identifier, IntConst, StringConst }

pub struct JackTokenizer {
    pub tokens: Vec<(TokenType, String)>,
    pub current: usize,
}

impl JackTokenizer {
    pub fn new(input: String) -> Self {
        let mut tokens = Vec::new();
        let keywords = ["class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else", "while", "return"];
        let symbols = "{}()[].,;+-*/&|<>=~";
        
        let mut chars = input.chars().peekable();
        
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else if c == '/' && chars.clone().nth(1) == Some('/') { // Line Comment
                while let Some(nc) = chars.next() { if nc == '\n' { break; } }
            } else if c == '"' { // String Constant
                chars.next(); // skip opening "
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '"' { chars.next(); break; }
                    s.push(chars.next().unwrap());
                }
                tokens.push((TokenType::StringConst, s));
            } else if symbols.contains(c) { // Symbol
                tokens.push((TokenType::Symbol, chars.next().unwrap().to_string()));
            } else if c.is_digit(10) { // Integer
                let mut num = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_digit(10) { num.push(chars.next().unwrap()); } else { break; }
                }
                tokens.push((TokenType::IntConst, num));
            } else { // Keyword or Identifier[cite: 4]
                let mut ident = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' { ident.push(chars.next().unwrap()); } else { break; }
                }
                if keywords.contains(&ident.as_str()) {
                    tokens.push((TokenType::Keyword, ident));
                } else {
                    tokens.push((TokenType::Identifier, ident));
                }
            }
        }
        JackTokenizer { tokens, current: 0 }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<tokens>\n");
        for (tt, val) in &self.tokens {
            let tag = match tt {
                TokenType::Keyword => "keyword",
                TokenType::Symbol => "symbol",
                TokenType::Identifier => "identifier",
                TokenType::IntConst => "integerConstant",
                TokenType::StringConst => "stringConstant",
            };
            // Replace XML special characters for symbols[cite: 4]
            let escaped = val.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");
            xml.push_str(&format!("  <{0}> {1} </{0}>\n", tag, escaped));
        }
        xml.push_str("</tokens>");
        xml
    }
}