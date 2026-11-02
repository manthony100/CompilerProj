// Main function
fn main() {
    let source = "#hai #obTw this is a comment #TLDR
     #MAEK HEAD #GIMMEH TITLE testing #OIC #MKAY
     #MAEK PARAgraf #NEWLINE 
        #GIMMEH BOLD After this is a list #OIC
        #MAEK LIST #GIMMEH ITEM item1 #OIC #GIMMEH ITEM item1 #OIC #MKAY 
    #MKAY #KBYE";

    let mut lexer = Lexer::new(source);

    if let Err(message) = lexer.tokenize() {
        eprintln!("{}", message);
        std::process::exit(1);
    }

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse_lolcode() {
        Ok(()) => {
            println!("Syntax analysis succeeded.");
        }

        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}

/// Trait for a simple lolcompiler front-end.
/// Errors should cause immediate exit inside the implementation.
pub trait Compiler {
/// Begin the compilation process (entry point).
fn compile(&mut self, source: &str);
/// Get the next token from the lexical analyzer.
fn next_token(&mut self) -> String;
/// Run the syntax analyzer starting from <lolcode>.
fn parse(&mut self);
/// Get the current token being processed.
fn current_token(&self) -> String;
/// Set the current token (typically used internally).
fn set_current_token(&mut self, tok: String);
}

    // Phase 2 - Task #1: Lexical Analyzer


/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer { // Declares the required behavior for a lexical analyzer in the LOLCODE Markdown language.
/// Return the next character from the input.
/// If input is exhausted, should terminate the program.
fn get_char(&mut self) -> char;
/// Add a character to the current potential token.
fn add_char(&mut self, c: char);
/// Lookup a potential token to determine if it is valid.
/// Returns true if a valid token/lexeme, false otherwise.
fn lookup(&self, s: &str) -> bool;
}

// /// OPTION 2 - Trait for a recursive descent Syntax Analyzer
// /// over Vec<String>. Each function parses a nonterminal in
// /// the grammar. On error: return Err(message), on success: Ok(()).
// I chose option two because like why wouldn't I want an error message, nvm so apparently it just means it'd be as a separate message which is also cool
pub trait SyntaxAnalyzer { // Declares the required behavior for a syntax analyzer in the LOLCODE Markdown language.
fn parse_lolcode(&mut self) -> Result<(), String>; // Parses the <lolcode> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_head(&mut self) -> Result<(), String>; // Parses the <head> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_title(&mut self) -> Result<(), String>; // Parses the <title> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_comment(&mut self) -> Result<(), String>; // Parses the <comment> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_body(&mut self) -> Result<(), String>; // Parses the <body> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_paragraph(&mut self) -> Result<(), String>; // Parses the <paragraph> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_inner_paragraph(&mut self) -> Result<(), String>; // Parses the <inner_paragraph> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_inner_text(&mut self) -> Result<(), String>; // Parses the <inner_text> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_variable_define(&mut self) -> Result<(), String>; // Parses the <variable_define> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_variable_use(&mut self) -> Result<(), String>; // Parses the <variable_use> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_bold(&mut self) -> Result<(), String>; // Parses the <bold> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_list(&mut self) -> Result<(), String>; // Parses the <list> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_list_items(&mut self) -> Result<(), String>; // Parses the <list_items> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_inner_list(&mut self) -> Result<(), String>; // Parses the <inner_list> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_link(&mut self) -> Result<(), String>; // Parses the <link> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_newline(&mut self) -> Result<(), String>; // Parses the <newline> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
fn parse_text(&mut self) -> Result<(), String>; // Parses the <text> nonterminal in the grammar. Returns Ok(()) on success, or Err(message) on error.
}

//              Phase 2 - Task #1 

// Character by character lexical analyzer

// Partitions the lexemes of a source file in our LOLCODE Markdown language into tokens

pub struct Lexer { 
    input: Vec<char>,
    position: usize,
    current_lexeme: String,
    tokens: Vec<String>,
    annotations: Vec<String>,
    selectors: Vec<String>,
    line: usize,
    column: usize,
}


impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            current_lexeme: String::new(),
            tokens: Vec::new(),

            annotations: vec![
                "#HAI".into(),
                "#KBYE".into(),
                "#OBTW".into(),
                "#TLDR".into(),
                "#MAEK".into(),
                "#OIC".into(),
                "#GIMMEH".into(),
                "#MKAY".into(),
                "#NEWLINE".into(),
                "#IHAZ".into(),
                "#ITIZ".into(),
            ],

            selectors: vec![
                "HEAD".into(),
                "TITLE".into(),
                "PARAGRAF".into(),
                "BOLD".into(),
                "LIST".into(),
                "ITEM".into(),
                "LINX".into(),
                "VAR".into(),
            ],

            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<(), String> {
        self.position = 0;
        self.line = 1;
        self.column = 1;
        self.current_lexeme.clear();
        self.tokens.clear();

        while self.position < self.input.len() {
            self.current_lexeme.clear();

            let start_line = self.line;
            let start_column = self.column;

            if self.input[self.position] == '#' {
                // Read the annotation's initial '#'.
                let c = self.get_char();
                self.add_char(c);

                // Read the rest of the annotation.
                while self.position < self.input.len() {
                    let next = self.input[self.position];

                    if next.is_whitespace() || next == '#' {
                        break;
                    }

                    let c = self.get_char();
                    self.add_char(c);
                }

                let annotation = self.current_lexeme.clone();

                let is_annotation = annotation.starts_with('#')
                    && self.lookup(&annotation);

                if !is_annotation {
                    return Err(format!(
                        "Lexical error at line {}, column {}: \
                        unknown annotation '{}'.",
                        start_line, start_column, annotation
                    ));
                }

                // Normalize keywords, but not ordinary text.
                self.tokens.push(annotation.to_ascii_uppercase());

                let expects_selector =
                    annotation.eq_ignore_ascii_case("#MAEK")
                        || annotation.eq_ignore_ascii_case("#GIMMEH");

                if expects_selector {
                    // Whitespace here separates the annotation
                    // from its selector; it is not document text.
                    while self.position < self.input.len()
                        && self.input[self.position].is_whitespace()
                    {
                        self.get_char();
                    }

                    let selector_line = self.line;
                    let selector_column = self.column;

                    self.current_lexeme.clear();

                    while self.position < self.input.len() {
                        let next = self.input[self.position];

                        if next.is_whitespace() || next == '#' {
                            break;
                        }

                        let c = self.get_char();
                        self.add_char(c);
                    }

                    let selector = self.current_lexeme.clone();

                    let is_selector = self.selectors
                        .iter()
                        .any(|allowed| {
                            allowed.eq_ignore_ascii_case(&selector)
                        });

                    if !is_selector {
                        return Err(format!(
                            "Lexical error at line {}, column {}: \
                            expected a selector after '{}', found {:?}.",
                            selector_line,
                            selector_column,
                            annotation,
                            selector
                        ));
                    }

                    self.tokens.push(selector.to_ascii_uppercase());
                }
            } else {
                // Ordinary text continues until the next annotation.
                while self.position < self.input.len()
                    && self.input[self.position] != '#'
                {
                    let c = self.get_char();
                    self.add_char(c);
                }

                if !self.current_lexeme.is_empty() {
                    self.tokens.push(self.current_lexeme.clone());
                }
            }
        }

        self.current_lexeme.clear();
        Ok(())
    }
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        if self.position >= self.input.len() {
            eprintln!(
                "Unexpected end of input at line {}, column {}.",
                self.line,
                self.column
            );
            std::process::exit(1);
        }

        let c = self.input[self.position];
        self.position += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        self.annotations
            .iter()
            .any(|annotation| annotation.eq_ignore_ascii_case(s))
            || self.selectors
                .iter()
                .any(|selector| selector.eq_ignore_ascii_case(s))
    }
}



//              Phase 2 - Task #2 
// Recursive-descent parser for LOLCODE

// Builds an abstract syntax (i.e. parse) tree, conceptually
// Syntax Analyzer Rust trait
// The implementation of the abstract syntax tree may best be done using a stack(s) or an array
// list/vector(s). Any syntax errors encountered should be reported as output to the console with as much error
// information as possible
// If an error is encountered, no output file should be created.
pub struct Parser {
    tokens: Vec<String>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&str> {
        self.tokens
            .get(self.position)
            .map(|token| token.as_str())
    }

    fn next_token_is(&self, expected: &str) -> bool {
        self.tokens
            .get(self.position + 1)
            .map(|token| token.as_str())
            == Some(expected)
    }

    fn syntax_error(&self, expected: &str) -> String {
        match self.current_token() {
            Some(actual) => format!(
                "Syntax error at token {}: expected {}, found {:?}.",
                self.position + 1,
                expected,
                actual
            ),

            None => format!(
                "Syntax error at end of input: expected {}.",
                expected
            ),
        }
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        if self.current_token() == Some(expected) {
            self.position += 1;
            Ok(())
        } else {
            Err(self.syntax_error(&format!("{:?}", expected)))
        }
    }

    fn skip_formatting_whitespace(&mut self) {
        while let Some(token) = self.current_token() {
            if token.chars().all(|c| c.is_whitespace()) {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn parse_optional_variable_define(&mut self) -> Result<(), String> {
        self.skip_formatting_whitespace();

        if self.current_token() == Some("#IHAZ") {
            self.parse_variable_define()?;
        }

        Ok(())
    }

    fn parse_variable_name(&mut self) -> Result<(), String> {
        let valid = match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                let name = token.trim();

                !name.is_empty()
                    && name.chars().all(|c| c.is_ascii_alphabetic())
            }

            _ => false,
        };

        if !valid {
            return Err(self.syntax_error(
                "a variable name containing only A-Z or a-z"
            ));
        }

        self.position += 1;
        Ok(())
    }

    fn parse_list_item(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("ITEM")?;

        self.parse_optional_variable_define()?;
        self.parse_inner_list()?;

        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_content_element(
        &mut self,
        allow_paragraph: bool,
        allow_list: bool,
    ) -> Result<(), String> {
        match self.current_token() {
            Some("#OBTW") => self.parse_comment(),

            Some("#NEWLINE") => self.parse_newline(),

            Some("#MAEK") => {
                if self.next_token_is("PARAGRAF") && allow_paragraph {
                    self.parse_paragraph()
                } else if self.next_token_is("LIST") && allow_list {
                    self.parse_list()
                } else {
                    Err(self.syntax_error(
                        "an allowed #MAEK annotation in this context"
                    ))
                }
            }

            Some("#GIMMEH") => {
                if self.next_token_is("BOLD") {
                    self.parse_bold()
                } else if self.next_token_is("LINX") {
                    self.parse_link()
                } else if self.next_token_is("VAR") {
                    self.parse_variable_use()
                } else {
                    Err(self.syntax_error(
                        "#GIMMEH BOLD, #GIMMEH LINX, or #GIMMEH VAR"
                    ))
                }
            }

            Some(token) if !token.starts_with('#') => {
                self.parse_text()
            }

            _ => Err(self.syntax_error(
                "an allowed content element"
            )),
        }
    }
}

impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;

        self.skip_formatting_whitespace();

        while self.current_token() == Some("#OBTW") {
            self.parse_comment()?;
            self.skip_formatting_whitespace();
        }

        self.parse_head()?;

        self.parse_optional_variable_define()?;

        self.parse_body()?;

        self.expect("#KBYE")?;

        if self.current_token().is_some() {
            return Err(self.syntax_error(
                "end of input after #KBYE"
            ));
        }

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        let begins_head =
            self.current_token() == Some("#MAEK")
                && self.next_token_is("HEAD");

        if begins_head {
            self.expect("#MAEK")?;
            self.expect("HEAD")?;

            self.skip_formatting_whitespace();

            self.parse_title()?;

            self.skip_formatting_whitespace();

            self.expect("#MKAY")?;
        }

        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("TITLE")?;

        self.parse_text()?;

        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;

        self.parse_text()?;

        self.expect("#TLDR")?;
        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> {
        loop {
            match self.current_token() {
                Some("#KBYE") => return Ok(()),

                None => {
                    return Err(self.syntax_error(
                        "body content or closing #KBYE"
                    ));
                }

                _ => {
                    self.parse_content_element(true, true)?;
                }
            }
        }
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect("#MAEK")?;
        self.expect("PARAGRAF")?;

        self.parse_optional_variable_define()?;

        self.parse_inner_paragraph()?;

        self.expect("#MKAY")?;
        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        loop {
            match self.current_token() {
                Some("#MKAY") => return Ok(()),

                None => {
                    return Err(self.syntax_error(
                        "paragraph content or closing #MKAY"
                    ));
                }

                _ => {
                    self.parse_inner_text()?;
                }
            }
        }
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        self.parse_content_element(false, true)
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#IHAZ")?;

        self.parse_variable_name()?;

        self.expect("#ITIZ")?;

        self.parse_text()?;

        self.expect("#MKAY")?;
        Ok(())
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("VAR")?;

        self.parse_variable_name()?;

        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("BOLD")?;

        self.parse_text()?;

        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("#MAEK")?;
        self.expect("LIST")?;

        self.skip_formatting_whitespace();

        self.parse_list_item()?;

        self.parse_list_items()?;

        self.expect("#MKAY")?;
        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        self.skip_formatting_whitespace();

        while self.current_token() == Some("#GIMMEH")
            && self.next_token_is("ITEM")
        {
            self.parse_list_item()?;
            self.skip_formatting_whitespace();
        }

        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        loop {
            match self.current_token() {
                Some("#OIC") => return Ok(()),

                None => {
                    return Err(self.syntax_error(
                        "list-item content or closing #OIC"
                    ));
                }

                _ => {
                    self.parse_content_element(false, false)?;
                }
            }
        }
    }

    fn parse_link(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("LINX")?;

        let valid_address = match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                let address = token.trim();

                !address.is_empty()
                    && !address.chars().any(|c| c.is_whitespace())
            }

            _ => false,
        };

        if !valid_address {
            return Err(self.syntax_error(
                "a nonempty address without internal whitespace"
            ));
        }

        self.position += 1;

        self.expect("#OIC")?;
        Ok(())
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")
    }

    fn parse_text(&mut self) -> Result<(), String> {
        match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                self.position += 1;
                Ok(())
            }

            _ => Err(self.syntax_error("plain text")),
        }
    }
}

//              Phase 2 - Task #3
// Implement the static scope variable resolution for the LOLCODE Markdown language
// Any static semantic errors - variable being used before it is defined -
// encountered should be reported as output to the console with as much error information as possible
// My compiler should exit after the first error is encountered. If an error is encountered, no output file should be created.

//              Phase 2 - Task #4
// Implement the semantic analyzer that takes the abstract syntax tree data structure and translates it to a
// lower-level, executable language - HTML
// Produce an output file with the same name as the input file, but with a .html extension
// The output file should be a valid HTML file that can be opened in a web browser and should contain the compiled html tags

//              Phase 3
// The final program should take only one command-line argument provided by the user -
// An input file name in the LOLCODE Markdown language

//              Phase 4
// Come up with test input files

//             Phase 5
// Document your code - already on it _( * *)__/ (took me a second to figure out how to do that lol, wanted to make it a salute but wtvr)

//             Phase 6
// Create public documentation using rustdoc