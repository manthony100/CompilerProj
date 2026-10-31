// Main function
fn main() {
    //     test for lexical part

    let mut lexer = Lexer::new("#HAI");

    let c = lexer.get_char();
    lexer.add_char(c);

    println!("First character: {:?}", c);
    println!("Current lexeme: {:?}", lexer.current_lexeme);
    println!("Recognizes #hai: {}", lexer.lookup("#hai"));
    // Comment for professor if you see on GitHub: thank god this part finally works (╥﹏╥)
    }
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
pub trait SyntaxAnalyzer {
fn parse_lolcode(&mut self) -> Result<(), String>;
fn parse_head(&mut self) -> Result<(), String>;
fn parse_title(&mut self) -> Result<(), String>;
fn parse_comment(&mut self) -> Result<(), String>;
fn parse_body(&mut self) -> Result<(), String>;
fn parse_paragraph(&mut self) -> Result<(), String>;
fn parse_inner_paragraph(&mut self) -> Result<(), String>;
fn parse_inner_text(&mut self) -> Result<(), String>;
fn parse_variable_define(&mut self) -> Result<(), String>;
fn parse_variable_use(&mut self) -> Result<(), String>;
fn parse_bold(&mut self) -> Result<(), String>;
fn parse_list(&mut self) -> Result<(), String>;
fn parse_list_items(&mut self) -> Result<(), String>;
fn parse_inner_list(&mut self) -> Result<(), String>;
fn parse_link(&mut self) -> Result<(), String>;
fn parse_newline(&mut self) -> Result<(), String>;
fn parse_text(&mut self) -> Result<(), String>;
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



        // TO DO Maybe Delete past lexical analyzer code and implement the new one above
// // implement of the LexicalAnalyzer struct - the actual executable code for the lexical analyzer
// pub struct LexericalAnalyzer { // gives the template / inteferfacce, structure
//     input: Vec<char>, // The input string as a vector of characters
//     position: usize, // The current position in the input string
//     current_lexeme: String, // The current lexeme being analyzed (built up character by character)
//     tokens: Vec<String>, // The vector of tokens generated by the lexical analyzer
//     line: usize, // The current line number in the input string - used for error reporting
//     column: usize, // The current column number in the input string - used for error reporting
//     // initialize the tokens vector to an empty vector
//     // Terminal tokens


//     // Fields for the lexical analyzer implementation
//     // For example, you might have a field for the input string, current position, etc.
// }

// impl LexicalAnalyzer { // The implementation of the LexicalAnalyzer struct
//     // Constructor for the LexicalAnalyzer struct
//     pub fn new(input: &str) -> Self { // create a new instance of the LexicalAnalyzer struct with the given input string
//         let mut lexer: LexicalAnalyzer = Self {
//             input: input.chars().collect(), // Convert the input string to a vector of characters
//             position: 0, // Initialize the position to 0
//             current_lexeme: String::new(), // Initialize the current lexeme to an empty string
//             tokens: Vec::new(), // Initialize the tokens vector to an empty vector
            
//             annotations: vec![
//                 "#HAI".into(),
//                 "#KBYE".into(),
//                 "#OBTW".into(),
//                 "#TLDR".into(),
//                 "#MAEK".into(),
//                 "#OIC".into(),
//                 "#GIMMEH".into(),
//                 "#MKAY".into(),
//                 "#NEWLINE".into(),
//                 "#IHAZ".into(),
//                 "#ITIZ".into(),
//             ],

//             selectors: vec![
//                 "HEAD".into(),
//                 "TITLE".into(),
//                 "PARAGRAF".into(),
//                 "BOLD".into(),
//                 "LIST".into(),
//                 "ITEM".into(),
//                 "LINX".into(),
//                 "VAR".into(),
//             ],

//             line: 1, // Initialize the line number to 1
//             column: 1, // Initialize the column number to 1
//         };

//         lexer.tokenize(); // Call the tokenize method to generate the tokens from the input string
//         lexer // Return the new instance of the LexicalAnalyzer struct
//     // Implement the methods for the LexicalAnalyzer struct here
//     // For example, you might have methods to get the next character, add a character to the current lexeme, etc.
//     }

//     pub fn get_char(&mut self) -> char { // Get the next character from the input string
//         if self.position < self.input.len() { // If the current position is less than the length of the input string
//             let c = self.input[self.position]; // Get the character at the current position
//             self.position += 1; // Increment the position to move to the next character
//             c
//         } else {
//             // If input is exhausted, terminate the program
//             // eprintln!("End of input reached.");
//             // std::process::exit(1);
//             '\0' // Return null character to indicate end of input
//         }
//     }

//     pub fn add_char(&mut self, c: char) { // Add a character to the current lexeme being built
//         if self.position < self.input.len() { // If the current position is less than the length of the input string
//             let c = self.input[self.position]; // Get the character at the current position
//             self.position += 1; // Increment the position to move to the next character
//             c
//         } else {
//             // If input is exhausted, terminate the program
//             // eprintln!("End of input reached.");
//             // std::process::exit(1);
//             '\0' // Return null character to indicate end of input
//         }
//     }

//         pub fn tokenize(&mut self) { // Tokenize the input string into tokens
//         loop { // Loop until the end of the input string is reached
//             let c = self.get_char(); // Get the next character from the input string

//             if c == '\0' { // If the end of the input string is reached, break out of the loop
//                 break; // Break out of the loop
//             }

//             if c.is_whitespace() { // If the character is whitespace, check if the current lexeme is not empty
//                 if !self.current_build.is_empty() { // If the current lexeme is not empty, push it to the tokens vector and clear the current lexeme
//                     self.tokens 
//                         .push(std::mem::take(&mut self.current_build)); // Push the current lexeme to the tokens vector and clear the current lexeme
//                 }
//             } else { // If the character is not whitespace, add it to the current lexeme being built
//                 self.add_char(c);
//             }
//         }

//         if !self.current_build.is_empty() { // If the current lexeme is not empty after the loop, push it to the tokens vector
//             self.tokens
//                 .push(std::mem::take(&mut self.current_build));
//         }

//         self.tokens.reverse(); // Reverse the tokens vector to maintain the original order of tokens
//     }   

//     fn lookup(&self, s: &str) -> bool {
//     self.annotations
//         .iter()
//         .any(|annotation| annotation.eq_ignore_ascii_case(s))
//         || self.selectors
//             .iter()
//             .any(|selector| selector.eq_ignore_ascii_case(s))
//     }
// }   


//              Phase 2 - Task #2 
// Recursive-descent parser for LOLCODE

// Builds an abstract syntax (i.e. parse) tree, conceptually
// Syntax Analyzer Rust trait
// The implementation of the abstract syntax tree may best be done using a stack(s) or an array
// list/vector(s). Any syntax errors encountered should be reported as output to the console with as much error
// information as possible
// If an error is encountered, no output file should be created.

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