// Main function
fn main() {
    let source = "#HAI #OBTW this is a comment #TLDR #OBTW this is another comment #TLDR #MAEK HEAD #GIMMEH TITLE testing #OIC #MKAY #KBYE";

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
// fn main() {
//     // Comment for professor if you see on GitHub: thank god this part finally works (╥﹏╥)

//     let source = "#HAI#KBYE";

//     let mut lexer = Lexer::new(source);

//     match lexer.tokenize() {
//         Ok(()) => {
//             println!("Tokenization succeeded.");

//             for (index, token) in lexer.tokens.iter().enumerate() {
//                 println!("{}: {:?}", index, token);
//             }
//         }
//         Err(message) => {
//             eprintln!("{}", message);
//             std::process::exit(1);
//         }
//     }
// }
// fn main() {

//     Input for the LOLCODE program
//     let lolcodeInput: &str = "..........";

//     Create a new instance of the Compiler struct
//     let mut compiler = Compiler::new(lolcodeInput);
//     Create a new instance of the LexicalAnalyzer struct with the lolcode input
//     let mut lexer: LexicalAnalyzer = LexicalAnalyzer::new(lolcodeInput);

//     Initialize the current token with the first token from the lexer
//     let compiler.current_token = lexer.start();

//     Create a new instance of the SyntaxAnalyzer struct with references to the lexer and compiler
//     let mut syntax_analyzer = SyntaxAnalyzer::new(&lexer, &compiler);
//     Call the lolspeak method to parse the lolcode according to the grammar rules
//     parser.lolspeak();

//     Check if there are any remaining tokens or if the current token is not empty
//     if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
//             Print an error message if there are any additional tokens
//             eprintln!("A syntax error was encountered. Additional tokens found.");
//             Exit the program with an error code
//             std::process::exit(1);
//     }
// }

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

/// OPTION 1 - Trait for a recursive descent Syntax Analyzer
/// over Vec<String>. Each function parses a nonterminal in
/// the grammar. On error: exit immediately.
// pub trait SyntaxAnalyzer {
// fn parse_lolcode(&mut self);
// fn parse_head(&mut self);
// fn parse_title(&mut self);
// fn parse_comment(&mut self);
// fn parse_body(&mut self);
// fn parse_paragraph(&mut self);
// fn parse_inner_paragraph(&mut self);
// fn parse_inner_text(&mut self);
// fn parse_variable_define(&mut self);
// fn parse_variable_use(&mut self);
// fn parse_bold(&mut self);
// fn parse_list(&mut self);
// fn parse_list_items(&mut self);
// fn parse_inner_list(&mut self);
// fn parse_link(&mut self);
// fn parse_newline(&mut self);
// fn parse_text(&mut self);
// }
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

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        match self.current_token() {
            Some(actual) if actual == expected => {
                self.position += 1;
                Ok(())
            }

            Some(actual) => Err(format!(
                "Syntax error at token {}: expected {:?}, found {:?}.",
                self.position + 1,
                expected,
                actual
            )),

            None => Err(format!(
                "Syntax error: expected {:?}, but reached end of input.",
                expected
            )),
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

    fn not_implemented(&self, rule: &str) -> Result<(), String> {
        Err(format!(
            "Parser rule {} is not implemented yet; \
             current token is {:?}.",
            rule,
            self.current_token()
        ))
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
        self.skip_formatting_whitespace();

        if self.current_token() == Some("#IHAZ") {
            self.parse_variable_define()?;
            self.skip_formatting_whitespace();
        }

        self.parse_body()?;
        self.expect("#KBYE")?;

        if let Some(extra) = self.current_token() {
            return Err(format!(
                "Syntax error at token {}: \
                 unexpected content after #KBYE: {:?}.",
                self.position + 1,
                extra
            ));
        }

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        let begins_head =
            self.current_token() == Some("#MAEK")
                && self.tokens
                    .get(self.position + 1)
                    .map(|token| token.as_str())
                    == Some("HEAD");

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

    fn parse_body(&mut self) -> Result<(), String> {
        // For this first milestone, only an empty body is supported.
        if self.current_token() == Some("#KBYE") {
            Ok(())
        } else {
            self.not_implemented("<body> with content")
        }
    }

    fn parse_text(&mut self) -> Result<(), String> {
        match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                self.position += 1;
                Ok(())
            }

            Some(token) => Err(format!(
                "Syntax error at token {}: expected text, found {:?}.",
                self.position + 1,
                token
            )),

            None => Err(
                "Syntax error: expected text, but reached end of input."
                    .to_string()
            ),
        }
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;
        self.parse_text()?;
        self.expect("#TLDR")?;

        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.not_implemented("<paragraph>")
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        self.not_implemented("<inner-paragraph>")
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        self.not_implemented("<inner-text>")
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.not_implemented("<variable-define>")
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.not_implemented("<variable-use>")
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        self.not_implemented("<bold>")
    }

    fn parse_list(&mut self) -> Result<(), String> {
        self.not_implemented("<list>")
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        self.not_implemented("<list-items>")
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        self.not_implemented("<inner-list>")
    }

    fn parse_link(&mut self) -> Result<(), String> {
        self.not_implemented("<link>")
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")
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