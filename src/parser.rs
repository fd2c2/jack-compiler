use crate::tokenizer::{JackTokenizer, TokenType};

pub struct Parser {
    pub tokenizer: JackTokenizer,
    xml: String,
    indent: usize,
}

impl Parser {
    pub fn new(tokenizer: JackTokenizer) -> Self {
        Parser { tokenizer, xml: String::new(), indent: 0 }
    }

    fn write_tag(&mut self, tag: &str, open: bool) {
        if !open { self.indent -= 2; }
        
        // Fix: Ensure both branches return a String
        let tag_name = if open { 
            tag.to_string() 
        } else { 
            format!("/{}", tag) 
        };

        self.xml.push_str(&format!("{:indent$}<{}>\n", "", tag_name, indent = self.indent));
        
        if open { self.indent += 2; }
    }

    fn eat(&mut self) {
        let (ttype, value) = &self.tokenizer.tokens[self.tokenizer.current];
        let tag = match ttype {
            TokenType::Keyword => "keyword",
            TokenType::Symbol => "symbol",
            TokenType::Identifier => "identifier",
            TokenType::IntConst => "integerConstant",
            TokenType::StringConst => "stringConstant",
        };
        let escaped = value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");
        self.xml.push_str(&format!("{:indent$}<{}> {} </{}>\n", "", tag, escaped, tag, indent = self.indent));
        self.tokenizer.current += 1;
    }

    pub fn compile_class(&mut self) -> String {
        self.write_tag("class", true);
        self.eat(); // 'class'
        self.eat(); // className
        self.eat(); // '{'

        while self.tokenizer.current < self.tokenizer.tokens.len() {
            let (_, value) = &self.tokenizer.tokens[self.tokenizer.current];
            if value == "}" { break; }

            match value.as_str() {
                "static" | "field" => self.compile_class_var_dec(),
                "function" | "method" | "constructor" => self.compile_subroutine(),
                _ => self.eat(),
            }
        }

        self.eat(); // '}'
        self.write_tag("class", false);
        self.xml.clone()
    }

    fn compile_class_var_dec(&mut self) {
        self.write_tag("classVarDec", true);
        while self.tokenizer.tokens[self.tokenizer.current].1 != ";" {
            self.eat();
        }
        self.eat(); // ';'[cite: 4]
        self.write_tag("classVarDec", false);
    }

    fn compile_subroutine(&mut self) {
        self.write_tag("subroutineDec", true);
        while self.tokenizer.tokens[self.tokenizer.current].1 != "{" {
            if self.tokenizer.tokens[self.tokenizer.current].1 == "(" {
                self.eat(); // '('
                self.write_tag("parameterList", true);
                while self.tokenizer.tokens[self.tokenizer.current].1 != ")" { self.eat(); }
                self.write_tag("parameterList", false);
                self.eat(); // ')'
            } else {
                self.eat();
            }
        }
        
        self.write_tag("subroutineBody", true);
        self.eat(); // '{'
        
        while self.tokenizer.tokens[self.tokenizer.current].1 == "var" {
            self.write_tag("varDec", true);
            while self.tokenizer.tokens[self.tokenizer.current].1 != ";" { self.eat(); }
            self.eat(); // ';'
            self.write_tag("varDec", false);
        }

        self.compile_statements();
        
        self.eat(); // '}'
        self.write_tag("subroutineBody", false);
        self.write_tag("subroutineDec", false);
    }

    fn compile_statements(&mut self) {
        self.write_tag("statements", true);
        loop {
            let (_, val) = &self.tokenizer.tokens[self.tokenizer.current];
            match val.as_str() {
                "let" => self.compile_let(),
                "do" => self.compile_do(),
                "while" => self.compile_while(),
                "if" => self.compile_if(),
                "return" => self.compile_return(),
                _ => break,
            }
        }
        self.write_tag("statements", false);
    }

    fn compile_let(&mut self) {
        self.write_tag("letStatement", true);
        while self.tokenizer.tokens[self.tokenizer.current].1 != ";" { self.eat(); }
        self.eat(); // ';'[cite: 4]
        self.write_tag("letStatement", false);
    }

    fn compile_do(&mut self) {
        self.write_tag("doStatement", true);
        while self.tokenizer.tokens[self.tokenizer.current].1 != ";" { self.eat(); }
        self.eat(); // ';'[cite: 4]
        self.write_tag("doStatement", false);
    }

    fn compile_while(&mut self) {
        self.write_tag("whileStatement", true);
        self.eat(); // 'while'
        self.eat(); // '('
        self.compile_expression();
        self.eat(); // ')'
        self.eat(); // '{'
        self.compile_statements();
        self.eat(); // '}'
        self.write_tag("whileStatement", false);
    }

    fn compile_if(&mut self) {
        self.write_tag("ifStatement", true);
        self.eat(); // 'if'
        self.eat(); // '('
        self.compile_expression();
        self.eat(); // ')'
        self.eat(); // '{'
        self.compile_statements();
        self.eat(); // '}'
        if self.tokenizer.tokens[self.tokenizer.current].1 == "else" {
            self.eat(); // 'else'
            self.eat(); // '{'
            self.compile_statements();
            self.eat(); // '}'
        }
        self.write_tag("ifStatement", false);
    }

    fn compile_return(&mut self) {
        self.write_tag("returnStatement", true);
        while self.tokenizer.tokens[self.tokenizer.current].1 != ";" { self.eat(); }
        self.eat(); // ';'[cite: 4]
        self.write_tag("returnStatement", false);
    }

    fn compile_expression(&mut self) {
        self.write_tag("expression", true);
        // Simple logic for Assignment 2: wrap the tokens in a term
        self.write_tag("term", true);
        while ![" ", ")", "]", ";", ",", "{", "}"].contains(&self.tokenizer.tokens[self.tokenizer.current].1.as_str()) {
            self.eat();
        }
        self.write_tag("term", false);
        self.write_tag("expression", false);
    }
}  