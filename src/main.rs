use std::env;
use std::fs;
use std::path::Path;
use std::process;

// Main function
fn main() {
    if let Err(message) = run() {
        eprintln!("{}", message);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(
            "Usage: lolcompiler <input.lol>".to_string()
        );
    }

    let input_path = Path::new(&args[1]);

    if input_path.extension().and_then(|ext| ext.to_str())
        != Some("lol")
    {
        return Err(
            "Usage error: input file must have a .lol extension."
                .to_string()
        );
    }

    let source = fs::read_to_string(input_path)
        .map_err(|error| {
            format!(
                "Could not read '{}': {}",
                input_path.display(),
                error
            )
        })?;

    let mut lexer = Lexer::new(&source);
    lexer.tokenize()?;

    let mut parser = Parser::new(lexer.tokens);
    parser.parse_lolcode()?;

    let tree = parser.nodes.first_mut().ok_or_else(|| {
        "Internal compiler error: no AST was produced.".to_string()
    })?;

    let mut resolver = ScopeResolver::new();
    resolver.resolve(tree)?;

    let html = HtmlGenerator::generate(tree)?;

    let output_path = input_path.with_extension("html");

    fs::write(&output_path, html)
        .map_err(|error| {
            format!(
                "Could not write '{}': {}",
                output_path.display(),
                error
            )
        })?;

    println!(
        "Compilation succeeded: {}",
        output_path.display()
    );

    if let Err(message) = open_in_browser(&output_path) {
        eprintln!("Warning: {}", message);
    }

    Ok(())
}

fn open_in_browser(path: &Path) -> Result<(), String> { // Attempt to open the generated HTML file in the default web browser.
    let absolute_path = fs::canonicalize(path) // Get the absolute path of the generated HTML file.
        .map_err(|error| { // If the file cannot be located, return an error message.
            format!(
                "Could not locate generated HTML file: {}",
                error
            )
        })?;

    let status = process::Command::new("open") // Use the "open" command to open the file in the default web browser.
        .arg(&absolute_path) // Add the absolute path of the HTML file as an argument to the "open" command.
        .status() 
        .map_err(|error| { // If the command cannot be launched, return an error message.
            format!(
                "Could not launch the browser-opening command: {}",
                error
            )
        })?;

    if !status.success() { // If the command fails, return an error message with the status code.
        return Err(format!(
            "Browser-opening command failed with status {}.",
            status
        ));
    }

    Ok(())
}
// fn main() {
//     let source = "#HAI
//         #MAEK LIST
//         #GIMMEH ITEM
//         #IHAZ itemname #ITIZ Homer #MKAY
//         #GIMMEH VAR itemname #OIC
//         #OIC
//         #GIMMEH ITEM
//         #GIMMEH VAR itemname #OIC
//         #OIC
//         #MKAY
//     #KBYE";

//     let mut lexer = Lexer::new(source);

//     if let Err(message) = lexer.tokenize() {
//         eprintln!("{}", message);
//         std::process::exit(1);
//     }

//     let mut parser = Parser::new(lexer.tokens);

//     if let Err(message) = parser.parse_lolcode() {
//         eprintln!("{}", message);
//         std::process::exit(1);
//     }

//     println!("Syntax analysis succeeded.");

//     let tree = match parser.nodes.first_mut() {
//         Some(tree) => tree,

//         None => {
//             eprintln!(
//                 "Internal compiler error: parsing produced no AST."
//             );
//             std::process::exit(1);
//         }
//     };

//     let mut resolver = ScopeResolver::new();

//     if let Err(message) = resolver.resolve(tree) {
//         eprintln!("{}", message);
//         std::process::exit(1);
//     }

//     println!("Static scope analysis succeeded.");
//     println!("{:#?}", tree);
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

pub struct Lexer { // Declares the structure of the lexical analyzer, which holds the input characters, current position, current lexeme being built, list of tokens, annotations, selectors, and line/column tracking for error reporting.
    input: Vec<char>,
    position: usize,
    current_lexeme: String,
    tokens: Vec<String>,
    annotations: Vec<String>,
    selectors: Vec<String>,
    line: usize,
    column: usize,
}


impl Lexer { // Implements the behavior of the lexical analyzer, including initialization, tokenization, and character handling.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(), // Converts the input string into a vector of characters for processing.
            position: 0, // Initializes the current position in the input to 0 (the start of the input).
            current_lexeme: String::new(), // Initializes the current lexeme being built to an empty string.
            tokens: Vec::new(), // Initializes the list of tokens to an empty vector.

            annotations: vec![ // Initializes the list of valid annotations in the LOLCODE Markdown language.
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

            selectors: vec![ // Initializes the list of valid selectors in the LOLCODE Markdown language.
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

    pub fn tokenize(&mut self) -> Result<(), String> { // Tokenizes the input characters into a list of tokens, handling annotations, selectors, and ordinary text, while tracking line and column numbers for error reporting.
        self.position = 0;
        self.line = 1;
        self.column = 1;
        self.current_lexeme.clear();
        self.tokens.clear();

        while self.position < self.input.len() { // Processes the input characters until the end of the input is reached.
            self.current_lexeme.clear(); // Clears the current lexeme being built for the next token.

            let start_line = self.line; // Records the starting line number for error reporting.
            let start_column = self.column; // Records the starting column number for error reporting.

            if self.input[self.position] == '#' { // If the current character is a '#', indicating the start of an annotation or selector.
                // Read the annotation's initial '#'.
                let c = self.get_char(); // Gets the current character (which is '#') and advances the position in the input.
                self.add_char(c); // Adds the '#' character to the current lexeme being built.

                // Read the rest of the annotation.
                while self.position < self.input.len() { // Continues reading characters until the end of the input is reached or a whitespace or another '#' is encountered.
                    let next = self.input[self.position]; // Peeks at the next character in the input without advancing the position.

                    if next.is_whitespace() || next == '#' { // If the next character is whitespace or another '#', the annotation reading is complete.
                        break;
                    }

                    let c = self.get_char(); // Gets the next character and advances the position in the input.
                    self.add_char(c); // Adds the character to the current lexeme being built.
                }

                let annotation = self.current_lexeme.clone(); // Clones the current lexeme to store the complete annotation for validation.

                let is_annotation = annotation.starts_with('#') // Checks if the current lexeme starts with a '#' and is a valid annotation or selector by looking it up in the predefined lists of annotations and selectors.
                    && self.lookup(&annotation);

                if !is_annotation { // If the current lexeme is not a valid annotation or selector, returns an error message indicating a lexical error with the line and column numbers, and the unknown annotation.
                    return Err(format!( // Returns an error message indicating a lexical error with the line and column numbers, and the unknown annotation.
                        "Lexical error at line {}, column {}: \
                        unknown annotation '{}'.",
                        start_line, start_column, annotation
                    ));
                }

                // Normalize keywords, but not ordinary text.
                self.tokens.push(annotation.to_ascii_uppercase()); // Adds the normalized annotation (converted to uppercase) to the list of tokens.

                let expects_selector = // Determines if the current annotation expects a selector to follow it, based on the specific annotations that require selectors.
                    annotation.eq_ignore_ascii_case("#MAEK")
                        || annotation.eq_ignore_ascii_case("#GIMMEH");

                if expects_selector { // If the current annotation expects a selector to follow it, the lexer will read the selector after skipping any whitespace.
                    // Whitespace here separates the annotation
                    // from its selector; it is not document text.
                    while self.position < self.input.len() // Skips any whitespace characters that separate the annotation from its selector, as whitespace is not considered part of the document text in this context.
                        && self.input[self.position].is_whitespace()
                    {
                        self.get_char();
                    }

                    let selector_line = self.line; // Records the line number where the selector is expected for error reporting.
                    let selector_column = self.column; // Records the column number where the selector is expected for error reporting.

                    self.current_lexeme.clear(); // Clears the current lexeme to prepare for reading the selector.

                    while self.position < self.input.len() { // Continues reading characters for the selector until the end of the input is reached or a whitespace or another '#' is encountered.
                        let next = self.input[self.position];

                        if next.is_whitespace() || next == '#' {
                            break;
                        }

                        let c = self.get_char();
                        self.add_char(c);
                    }

                    let selector = self.current_lexeme.clone(); // Clones the current lexeme to store the complete selector for validation.

                    let is_selector = self.selectors // Checks if the current lexeme is a valid selector by looking it up in the predefined list of selectors, ignoring case.
                        .iter()
                        .any(|allowed| {
                            allowed.eq_ignore_ascii_case(&selector)
                        });

                    if !is_selector { // If the current lexeme is not a valid selector, returns an error message indicating a lexical error with the line and column numbers, the annotation that expected a selector, and the unknown selector.
                        return Err(format!(
                            "Lexical error at line {}, column {}: \
                            expected a selector after '{}', found {:?}.",
                            selector_line,
                            selector_column,
                            annotation,
                            selector
                        ));
                    }

                    self.tokens.push(selector.to_ascii_uppercase()); // Adds the normalized selector (converted to uppercase) to the list of tokens.
                }
            } else {
                // Ordinary text continues until the next annotation.
                while self.position < self.input.len() // Continues reading characters for ordinary text until the end of the input is reached or a '#' is encountered, which indicates the start of an annotation.
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

        self.current_lexeme.clear(); // Clears the current lexeme after processing all input characters, preparing for any further processing if needed.
        Ok(())
    }
}

impl LexicalAnalyzer for Lexer { // Implements the required behavior for the lexical analyzer, including getting the next character, adding a character to the current lexeme, and looking up potential tokens for validation.
    fn get_char(&mut self) -> char { // Returns the next character from the input, advancing the position and updating line and column numbers for error reporting. If the end of input is reached, it prints an error message and exits the program.
        if self.position >= self.input.len() { // If the current position is beyond the end of the input, it indicates that there are no more characters to read, and an error message is printed to the console with the current line and column numbers, followed by exiting the program.
            eprintln!(
                "Unexpected end of input at line {}, column {}.",
                self.line,
                self.column
            );
            std::process::exit(1);
        }

        let c = self.input[self.position]; // Retrieves the character at the current position in the input.
        self.position += 1; // Advances the position in the input to the next character.

        if c == '\n' { // If the character is a newline, it indicates that the line number should be incremented and the column number reset to 1 for the start of the new line.
            self.line += 1; // Increments the line number to reflect that a new line has been encountered in the input.
            self.column = 1; // Resets the column number to 1, as the next character will be at the start of the new line.
        } else {
            self.column += 1; // Increments the column number to reflect that a character has been read on the current line, moving the cursor to the right.
        }

        c // Returns the character that was read from the input.
    }

    fn add_char(&mut self, c: char) { // Adds a character to the current potential token (lexeme) being built by appending it to the current_lexeme string.
        self.current_lexeme.push(c);
    }

    fn lookup(&self, s: &str) -> bool { // Checks if a given string is a valid annotation or selector by looking it up in the predefined lists of annotations and selectors, ignoring case. Returns true if the string is found in either list, and false otherwise.
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
#[derive(Debug, PartialEq)] // Represents the different types of nodes in the abstract syntax tree for the LOLCODE Markdown language.
pub enum AstNode { // Represents the different types of nodes in the abstract syntax tree for the LOLCODE Markdown language.
    Document(Vec<AstNode>),
    Head(Vec<AstNode>),
    Paragraph(Vec<AstNode>),
    List(Vec<AstNode>),
    Item(Vec<AstNode>),

    Title(String),
    Comment(String),
    Bold(String),
    Text(String),
    Link(String),
    Newline,

    VariableDefine { // Represents a variable definition in the abstract syntax tree, containing the variable's name and value.
        name: String,
        value: String,
    },

    VariableUse { // Represents a variable usage in the abstract syntax tree, containing the variable's name.
        name: String,
    },
}


pub struct Parser { // Represents the recursive-descent parser for the LOLCODE Markdown language, holding the list of tokens, the current position in the token list, and the nodes of the abstract syntax tree.
    tokens: Vec<String>,
    position: usize,
    nodes: Vec<AstNode>, // Added a field to hold the nodes of the abstract syntax tree
}

impl Parser { // Implements the behavior of the recursive-descent parser, including initialization, token handling, and parsing functions for different nonterminals in the grammar.
    pub fn new(tokens: Vec<String>) -> Self { // Initializes a new instance of the Parser with the provided list of tokens, setting the initial position to 0 and initializing the nodes vector to hold the abstract syntax tree.
        Self {
            tokens,
            position: 0,
            nodes: Vec::new(), // Added this - Initializes the nodes vector to hold the abstract syntax tree
        }
    }

    fn current_token(&self) -> Option<&str> { // Returns the current token being processed by the parser, or None if the end of the token list has been reached. This function is used to check the current token without advancing the position in the token list.
        self.tokens
            .get(self.position)
            .map(|token| token.as_str())
    }

    fn next_token_is(&self, expected: &str) -> bool { // Checks if the next token in the token list matches the expected string, without advancing the position in the token list.
        self.tokens
            .get(self.position + 1)
            .map(|token| token.as_str())
            == Some(expected)
    }

    fn syntax_error(&self, expected: &str) -> String { // Generates a syntax error message indicating the expected token and the actual token found at the current position in the token list. If the end of input is reached, it indicates that the expected token was not found.
        match self.current_token() { // Checks the current token being processed by the parser.
            Some(actual) => format!( // If a token is found, it generates an error message indicating the expected token and the actual token found at the current position in the token list.
                "Syntax error at token {}: expected {}, found {:?}.",
                self.position + 1,
                expected,
                actual
            ),

            None => format!( // If the end of input is reached, it generates an error message indicating that the expected token was not found at the end of input.
                "Syntax error at end of input: expected {}.",
                expected
            ),
        }
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> { // Checks if the current token matches the expected string. If it does, it advances the position in the token list and returns Ok(()). If it does not match, it returns an error message indicating the expected token and the actual token found at the current position in the token list.
        if self.current_token() == Some(expected) { // Checks if the current token matches the expected string.
            self.position += 1; // Advances the position in the token list to the next token.
            Ok(()) // Returns Ok(()) to indicate that the expected token was found and processed successfully.
        } else { // If the current token does not match the expected string, it generates a syntax error message indicating the expected token and the actual token found at the current position in the token list, and returns it as an Err variant.
            Err(self.syntax_error(&format!("{:?}", expected)))
        }
    }

    fn skip_formatting_whitespace(&mut self) { // Skips any whitespace tokens in the token list, advancing the position until a non-whitespace token is found or the end of the token list is reached. This function is used to ignore formatting whitespace in the LOLCODE Markdown language.
        while let Some(token) = self.current_token() { // Continues to check the current token while there are tokens remaining in the token list.
            if token.chars().all(|c| c.is_whitespace()) {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn parse_optional_variable_define(&mut self) -> Result<(), String> { // Parses an optional variable definition in the LOLCODE Markdown language. If the current token is "#IHAZ", it calls the parse_variable_define function to parse the variable definition. If the current token is not "#IHAZ", it simply returns Ok(()) to indicate that no variable definition was found, and parsing can continue.
        self.skip_formatting_whitespace();

        if self.current_token() == Some("#IHAZ") { // Checks if the current token is "#IHAZ", indicating the start of a variable definition.
            self.parse_variable_define()?; // Calls the parse_variable_define function to parse the variable definition, and propagates any errors that may occur during parsing.
        }

        Ok(())
    }

    fn parse_variable_name(&mut self) -> Result<(), String> { // Parses a variable name in the LOLCODE Markdown language. It checks if the current token is a valid variable name (not starting with '#' and containing only ASCII alphabetic characters). If the variable name is valid, it advances the position in the token list. If the variable name is invalid, it returns an error message indicating that a valid variable name was expected.
        let valid = match self.current_token() { // Checks the current token being processed by the parser.
            Some(token) if !token.starts_with('#') => { // If the current token does not start with '#', it is considered a potential variable name.
                let name = token.trim(); // Trims any leading or trailing whitespace from the token to get the variable name.

                !name.is_empty() // Checks if the variable name is not empty and contains only ASCII alphabetic characters.
                    && name.chars().all(|c| c.is_ascii_alphabetic())
            }

            _ => false, // If the current token is None or starts with '#', it is considered invalid for a variable name.
        };

        if !valid { // If the variable name is invalid, it generates a syntax error message indicating that a valid variable name was expected, and returns it as an Err variant.
            return Err(self.syntax_error(
                "a variable name containing only A-Z or a-z"
            ));
        }

        self.position += 1; // Advances the position in the token list to the next token, as the variable name has been successfully parsed.
        Ok(())
    }

    fn parse_list_item(&mut self) -> Result<(), String> { // Parses a list item in the LOLCODE Markdown language. It expects the current token to be "#GIMMEH" followed by "ITEM". It then parses an optional variable definition and the inner content of the list item. Finally, it expects the closing token "#OIC" and finishes the container for the list item in the abstract syntax tree.
        self.expect("#GIMMEH")?; // Expects the current token to be "#GIMMEH"
        self.expect("ITEM")?; // Expects the next token to be "ITEM"

        let start = self.nodes.len(); // Remember the starting index of the list item

        self.parse_optional_variable_define()?; // Parses an optional variable definition within the list item
        self.parse_inner_list()?; // Parses the inner content of the list item

        self.expect("#OIC")?; // Expects the closing token "#OIC" for the list item
        self.finish_container(start, AstNode::Item); // Finishes the container for the list item in the abstract syntax tree, creating an AstNode::Item with the collected child nodes
        Ok(())
    }

    fn parse_content_element( // Parses a content element in the LOLCODE Markdown language. It checks the current token and determines which type of content element to parse based on the token. It handles comments, newlines, paragraphs, lists, bold text, links, variable uses, and ordinary text. The function takes two boolean parameters to allow or disallow paragraphs and lists in the current context.
        &mut self,
        allow_paragraph: bool, // Indicates whether paragraphs are allowed in the current context. If true, the parser will attempt to parse a paragraph when encountering a "#MAEK PARAGRAF" token sequence.
        allow_list: bool, // Indicates whether lists are allowed in the current context. If true, the parser will attempt to parse a list when encountering a "#MAEK LIST" token sequence.
    ) -> Result<(), String> {
        match self.current_token() { // Checks the current token being processed by the parser to determine which type of content element to parse.
            Some("#OBTW") => self.parse_comment(), // If the current token is "#OBTW", it calls the parse_comment function to parse a comment.

            Some("#NEWLINE") => self.parse_newline(), // If the current token is "#NEWLINE", it calls the parse_newline function to parse a newline.

            Some("#MAEK") => { // If the current token is "#MAEK", it checks the next token to determine whether to parse a paragraph or a list, based on the allow_paragraph and allow_list parameters.
                if self.next_token_is("PARAGRAF") && allow_paragraph { // If the next token is "PARAGRAF" and paragraphs are allowed in the current context, it calls the parse_paragraph function to parse a paragraph.
                    self.parse_paragraph()
                } else if self.next_token_is("LIST") && allow_list { // If the next token is "LIST" and lists are allowed in the current context, it calls the parse_list function to parse a list.
                    self.parse_list()
                } else { // If neither a paragraph nor a list is allowed in the current context, it returns an error message indicating that an allowed "#MAEK" annotation was expected in this context.
                    Err(self.syntax_error( 
                        "an allowed #MAEK annotation in this context"
                    ))
                }
            }

            Some("#GIMMEH") => { // If the current token is "#GIMMEH", it checks the next token to determine whether to parse bold text, a link, or a variable use.
                if self.next_token_is("BOLD") {
                    self.parse_bold() // If the next token is "BOLD", it calls the parse_bold function to parse bold text.
                } else if self.next_token_is("LINX") {
                    self.parse_link() // If the next token is "LINX", it calls the parse_link function to parse a link.
                } else if self.next_token_is("VAR") {
                    self.parse_variable_use() // If the next token is "VAR", it calls the parse_variable_use function to parse a variable use.
                } else {
                    Err(self.syntax_error( // If none of the expected tokens follow "#GIMMEH", it returns an error message indicating that one of the allowed "#GIMMEH" annotations was expected in this context.
                        "#GIMMEH BOLD, #GIMMEH LINX, or #GIMMEH VAR"
                    ))
                }
            }

            Some(token) if !token.starts_with('#') => { // If the current token is ordinary text (not starting with '#'), it calls the parse_text function to parse the text content.
                self.parse_text()
            }

            _ => Err(self.syntax_error( // If none of the expected tokens match the current token, it returns an error message indicating that an allowed content element was expected in this context.
                "an allowed content element"
            )),
        }
    }

    fn take_plain_text(&mut self) -> Result<String, String> { // Takes plain text from the current token in the token list. If the current token is ordinary text (not starting with '#'), it returns the text as a String and advances the position in the token list. If the current token is not ordinary text, it returns an error message indicating that plain text was expected.
        match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                let text = token.to_string();
                self.position += 1;
                Ok(text)
            }

            _ => Err(self.syntax_error("plain text")),
        }
    }

    fn finish_container( // Finishes a container node in the abstract syntax tree by collecting child nodes from the nodes vector starting from the specified index
        &mut self,
        start: usize,
        make_node: fn(Vec<AstNode>) -> AstNode,
    ) {
        let children = self.nodes.split_off(start); //
        self.nodes.push(make_node(children)); // Creates a new AST node using the provided function and the collected child nodes, and adds it to the nodes vector.
    }
}

impl SyntaxAnalyzer for Parser { // Implements the required behavior for the syntax analyzer, including parsing functions for different nonterminals in the grammar. Each function returns a Result indicating success or failure, with an error message in case of failure.
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?; // Expects the current token to be "#HAI", indicating the start of a LOLCODE document.

        let start = self.nodes.len(); // Remember the starting index of the document node in the abstract syntax tree.

        self.skip_formatting_whitespace();

        while self.current_token() == Some("#OBTW") { // While the current token is "#OBTW", indicating the start of a comment, it calls the parse_comment function to parse the comment and then skips any formatting whitespace. (Can have multiple comments at the start of the document)
            self.parse_comment()?;
            self.skip_formatting_whitespace();
        }

        self.parse_head()?; // Optional Header

        self.parse_optional_variable_define()?; // Optional Variable Definition

        self.parse_body()?; // Parses the body of the document, which can contain various content elements such as paragraphs, lists, bold text, links, variable uses, and ordinary text.

        self.expect("#KBYE")?; // Expects the current token to be "#KBYE", indicating the end of the LOLCODE document.

        if self.current_token().is_some() { // If there are any tokens remaining after the "#KBYE" token, it indicates that there is unexpected content after the end of the document, and it returns an error message indicating that the end of input was expected after "#KBYE".
            return Err(self.syntax_error(
                "end of input after #KBYE"
            ));
        }

        self.finish_container(start, AstNode::Document); // Finishes the container for the document node in the abstract syntax tree, creating an AstNode::Document with the collected child nodes.

        Ok(()) // Returns Ok(()) to indicate that the parsing of the LOLCODE document was successful.
    }

    fn parse_head(&mut self) -> Result<(), String> { // Parses the head section of the LOLCODE document.
        let begins_head = // Checks if the current token is "#MAEK" and the next token is "HEAD", indicating the start of a head section in the document.
            self.current_token() == Some("#MAEK")
                && self.next_token_is("HEAD");

        if begins_head { // If the current token sequence indicates the start of a head section, it proceeds to parse the head section.
            self.expect("#MAEK")?;
            self.expect("HEAD")?;

            let start = self.nodes.len();

            self.skip_formatting_whitespace();

            self.parse_title()?; // Parses the title of the document, which is expected to be present in the head section.

            self.skip_formatting_whitespace();

            self.expect("#MKAY")?; // Expects the current token to be "#MKAY", indicating the end of the head section.

            self.finish_container(start, AstNode::Head); // Finishes the container for the head node in the abstract syntax tree, creating an AstNode::Head with the collected child nodes.
        }

        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> { // Parses the title of the LOLCODE document. 
        self.expect("#GIMMEH")?; // Expects the current token to be "#GIMMEH", indicating the start of a title section in the document.
        self.expect("TITLE")?; // Expects the next token to be "TITLE", indicating that the title is being defined.

        let text = self.take_plain_text()?; // Takes the plain text following the "#GIMMEH TITLE" tokens, which represents the title of the document.

        self.expect("#OIC")?; // Expects the current token to be "#OIC", indicating the end of the title section.

        self.nodes.push(AstNode::Title(text)); // Adds an AstNode::Title node to the nodes vector in the abstract syntax tree, containing the title text that was parsed.

        Ok(()) // Returns Ok(()) to indicate that the parsing of the title was successful.
    }

    fn parse_comment(&mut self) -> Result<(), String> { // Parses a comment in the LOLCODE document.

        let text = self.take_plain_text()?; // Takes the plain text following the "#OBTW" token, which represents the comment content.

        self.expect("#TLDR")?; // Expects the current token to be "#TLDR", indicating the end of the comment section.

        self.nodes.push(AstNode::Comment(text)); // Adds an AstNode::Comment node to the nodes vector in the abstract syntax tree, containing the comment text that was parsed.

        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> { // Parses the body of the LOLCODE document.
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

    fn parse_paragraph(&mut self) -> Result<(), String> { // Parses a paragraph in the LOLCODE document.
        self.expect("#MAEK")?; // Expects the current token to be "#MAEK", indicating the start of a paragraph section in the document.
        self.expect("PARAGRAF")?; // Expects the next token to be "PARAGRAF", indicating that a paragraph is being defined.

        let start = self.nodes.len();

        self.parse_optional_variable_define()?; // Parses an optional variable definition within the paragraph section, if present.

        self.parse_inner_paragraph()?; // Parses the inner content of the paragraph, which can contain various content elements such as bold text, links, variable uses, and ordinary text.

        self.expect("#MKAY")?; // Expects the current token to be "#MKAY", indicating the end of the paragraph section.

        self.finish_container(start, AstNode::Paragraph); // Finishes the container for the paragraph node in the abstract syntax tree, creating an AstNode::Paragraph with the collected child nodes.

        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> { // Parses the inner content of a paragraph in the LOLCODE document. It continues parsing content elements until it encounters the closing token "#MKAY" or reaches the end of input. If it encounters an unexpected token, it returns an error message indicating that paragraph content or the closing "#MKAY" was expected.
        loop {
            match self.current_token() {
                Some("#MKAY") => return Ok(()),

                None => {
                    return Err(self.syntax_error(
                        "paragraph content or closing #MKAY"
                    ));
                }

                _ => {
                    self.parse_inner_text()?; // Parses the inner text content of the paragraph, allowing for various content elements such as bold text, links, variable uses, and ordinary text.
                }
            }
        }
    }

    fn parse_inner_text(&mut self) -> Result<(), String> { // Parses the inner text content of a paragraph in the LOLCODE document. 
        self.parse_content_element(false, true) // Calls the parse_content_element function with allow_paragraph set to false and allow_list set to true, allowing for various content elements such as bold text, links, variable uses, and ordinary text, but disallowing nested paragraphs within the current paragraph context.
    }

    fn parse_variable_define(&mut self) -> Result<(), String> { // Parses a variable definition in the LOLCODE document. It expects the current token to be "#IHAZ", indicating the start of a variable
        self.expect("#IHAZ")?; // Expects the current token to be "#IHAZ", indicating the start of a variable definition.

        let name_position = self.position;

        self.parse_variable_name()?; // Parses the variable name following the "#IHAZ" token, ensuring that it is a valid variable name according to the defined rules.

        let name = self.tokens[name_position] // Retrieves the variable name from the tokens vector using the recorded position of the variable name token, trims any leading or trailing whitespace, and converts it to a String for use in the abstract syntax tree.
            .trim()
            .to_string();

        self.expect("#ITIZ")?; // Expects the current token to be "#ITIZ", indicating that the variable definition is followed by its value.

        let value = self.take_plain_text()?; // Takes the plain text following the "#ITIZ" token, which represents the value of the variable being defined.

        self.expect("#MKAY")?; // Expects the current token to be "#MKAY", indicating the end of the variable definition.

        self.nodes.push(AstNode::VariableDefine { // Adds an AstNode::VariableDefine node to the nodes vector in the abstract syntax tree, containing the variable's name and value that were parsed.
            name,
            value,
        });

        Ok(()) // Returns Ok(()) to indicate that the parsing of the variable definition was successful.
    }

    fn parse_variable_use(&mut self) -> Result<(), String> { // Parses a variable usage in the LOLCODE document. 
        self.expect("#GIMMEH")?; // Expects the current token to be "#GIMMEH", indicating the start of a variable usage.
        self.expect("VAR")?; // Expects the next token to be "VAR", indicating that a variable is being used.

        let name_position = self.position;

        self.parse_variable_name()?; // Parses the variable name following the "#GIMMEH VAR" tokens, ensuring that it is a valid variable name according to the defined rules.

        let name = self.tokens[name_position]
            .trim()
            .to_string();

        self.expect("#OIC")?; // Expects the current token to be "#OIC", indicating the end of the variable usage.

        self.nodes.push(AstNode::VariableUse { // Adds an AstNode::VariableUse node to the nodes vector in the abstract syntax tree, containing the variable's name that was parsed.
            name,
        });

        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> { // Parses bold text in the LOLCODE document.
        self.expect("#GIMMEH")?; // Expects the current token to be "#GIMMEH", indicating the start of a bold text section.
        self.expect("BOLD")?; // Expects the next token to be "BOLD", indicating that bold text is being defined.

        let text = self.take_plain_text()?; // Takes the plain text following the "#GIMMEH BOLD" tokens, which represents the bold text content.

        self.expect("#OIC")?; // Expects the current token to be "#OIC", indicating the end of the bold text section.

        self.nodes.push(AstNode::Bold(text)); // Adds an AstNode::Bold node to the nodes vector in the abstract syntax tree, containing the bold text that was parsed.

        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> { // Parses a list in the LOLCODE document.
        self.expect("#MAEK")?; // Expects the current token to be "#MAEK", indicating the start of a list section.
        self.expect("LIST")?; // Expects the next token to be "LIST", indicating that a list is being defined.

        let start = self.nodes.len(); 

        self.skip_formatting_whitespace();

        self.parse_list_item()?; // Parses the first list item in the list section, which is expected to be present.

        self.parse_list_items()?; // Parses any additional list items in the list section, allowing for multiple list items to be defined.

        self.expect("#MKAY")?; // Expects the current token to be "#MKAY", indicating the end of the list section.

        self.finish_container(start, AstNode::List); // Finishes the container for the list node in the abstract syntax tree, creating an AstNode::List with the collected child nodes.

        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> { // Parses additional list items in the LOLCODE document.
        self.skip_formatting_whitespace();

        while self.current_token() == Some("#GIMMEH") // While the current token is "#GIMMEH" and the next token is "ITEM", indicating the start of a list item
            && self.next_token_is("ITEM")
        {
            self.parse_list_item()?; // calls the parse_list_item function to parse the list item 
            self.skip_formatting_whitespace(); // and then skips any formatting whitespace. 
        } // This loop continues until there are no more list items to parse.

        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> { // Parses the inner content of a list item in the LOLCODE document. 
        loop {
            match self.current_token() {
                Some("#OIC") => return Ok(()), // It continues parsing content elements until it encounters the closing token "#OIC" 

                None => {
                    return Err(self.syntax_error( // If it reaches the end of input without finding the closing "#OIC" token, it returns an error message indicating that list-item content or the closing "#OIC" was expected.
                        "list-item content or closing #OIC"
                    ));
                }

                _ => { // If the current token is neither "#OIC" nor None, it calls the parse_content_element function to parse the content element within the list item
                }
            }
        }
    } //  If it encounters an unexpected token, it returns an error message indicating that list-item content or the closing "#OIC" was expected.

    fn parse_link(&mut self) -> Result<(), String> { // Parses a link in the LOLCODE document.
        self.expect("#GIMMEH")?; // Expects the current token to be "#GIMMEH", indicating the start of a link section.
        self.expect("LINX")?; // Expects the next token to be "LINX", indicating that a link is being defined.

        let address = match self.current_token() { // Checks the current token to determine if it is a valid link address. A valid link address is a nonempty string that does not start with '#' and does not contain any internal whitespace.
            Some(token) if !token.starts_with('#') => {
                let address = token.trim();

                if address.is_empty()
                    || address.chars().any(|c| c.is_whitespace())
                {
                    return Err(self.syntax_error(
                        "a nonempty address without internal whitespace"
                    ));
                }

                address.to_string() // If the current token is a valid link address, it returns the address as a String for use in the abstract syntax tree.
            }

            _ => { // If the current token is not a valid link address, it returns an error message indicating that a link address was expected.
                return Err(self.syntax_error("a link address"));
            }
        };

        self.position += 1;

        self.expect("#OIC")?; // Expects the current token to be "#OIC", indicating the end of the link section.

        self.nodes.push(AstNode::Link(address)); // Adds an AstNode::Link node to the nodes vector in the abstract syntax tree, containing the link address that was parsed.

        Ok(())
    }
 
    fn parse_newline(&mut self) -> Result<(), String> { // Parses a newline in the LOLCODE document.
        self.expect("#NEWLINE")?; // Expects the current token to be "#NEWLINE", indicating that a newline is being defined.

        self.nodes.push(AstNode::Newline); // Adds an AstNode::Newline node to the nodes vector in the abstract syntax tree, representing a newline in the document.

        Ok(())
    }

    fn parse_text(&mut self) -> Result<(), String> { // Parses ordinary text in the LOLCODE document.
        let text = self.take_plain_text()?; // Takes the plain text from the current token in the token list, which represents ordinary text content.

        self.nodes.push(AstNode::Text(text)); // Adds an AstNode::Text node to the nodes vector in the abstract syntax tree, containing the ordinary text that was parsed.

        Ok(())
    }
}


//              Phase 2 - Task #3
// Implement the static scope variable resolution for the LOLCODE Markdown language
// Any static semantic errors - variable being used before it is defined -
// encountered should be reported as output to the console with as much error information as possible
// My compiler should exit after the first error is encountered. If an error is encountered, no output file should be created.
pub struct ScopeResolver { // Represents the static scope variable resolver for the LOLCODE Markdown language.
    scopes: Vec<Vec<(String, String)>>, // Stack of scopes, each scope is a vector of (variable name, variable value) pairs
}

impl ScopeResolver { // Implements the behavior of the static scope variable resolver, including initialization, variable definition, variable lookup, and resolution of the abstract syntax tree.
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, tree: &mut AstNode) -> Result<(), String> { // Resolves the abstract syntax tree by clearing the current scopes, checking if the root node is a Document, and then calling the resolve_node function to perform variable resolution on the entire tree. If the root node is not a Document, it returns an error message indicating that a Document AST root was expected.
        self.scopes.clear();

        if !matches!(tree, AstNode::Document(_)) {
            return Err(
                "Static semantic error: expected a Document AST root."
                    .to_string()
            );
        }

        self.resolve_node(tree)
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_variable(
        &mut self,
        name: &str,
        value: &str,
    ) -> Result<(), String> {
        let scope = match self.scopes.last_mut() {
            Some(scope) => scope,

            None => {
                return Err(
                    "Internal resolver error: no active scope."
                        .to_string()
                );
            }
        };

        scope.push((
            name.to_string(),
            value.to_string(),
        ));

        Ok(())
    }

    fn lookup_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            for (defined_name, value) in scope.iter().rev() {
                if defined_name == name {
                    return Some(value.clone());
                }
            }
        }

        None
    }

    fn resolve_children(
        &mut self,
        children: &mut [AstNode],
    ) -> Result<(), String> {
        self.enter_scope();

        let result = (|| -> Result<(), String> {
            for child in children {
                self.resolve_node(child)?;
            }

            Ok(())
        })();

        self.leave_scope();

        result
    }

    fn resolve_node(
        &mut self,
        node: &mut AstNode,
    ) -> Result<(), String> {
        match node {
            AstNode::Document(children)
            | AstNode::Head(children)
            | AstNode::Paragraph(children)
            | AstNode::List(children)
            | AstNode::Item(children) => {
                self.resolve_children(children)
            }

            AstNode::VariableDefine { name, value } => {
                self.define_variable(name, value)
            }

            AstNode::VariableUse { name } => {
                let value = match self.lookup_variable(name) {
                    Some(value) => value,

                    None => {
                        return Err(format!(
                            "Static semantic error: variable '{}' \
                             is used before it is defined or is \
                             outside its scope.",
                            name
                        ));
                    }
                };

                *node = AstNode::Text(value);

                Ok(())
            }

            AstNode::Title(_)
            | AstNode::Comment(_)
            | AstNode::Bold(_)
            | AstNode::Text(_)
            | AstNode::Link(_)
            | AstNode::Newline => Ok(()),
        }
    }
}


//              Phase 2 - Task #4
// Implement the semantic analyzer that takes the abstract syntax tree data structure and translates it to a
// lower-level, executable language - HTML
// Produce an output file with the same name as the input file, but with a .html extension
// The output file should be a valid HTML file that can be opened in a web browser and should contain the compiled html tags
pub struct HtmlGenerator;

impl HtmlGenerator { // Implements the behavior of the HTML generator, including generating HTML from the abstract syntax tree, escaping HTML special characters, and rendering different types of nodes in the tree.
    pub fn generate(tree: &AstNode) -> Result<String, String> {
        let children = match tree {
            AstNode::Document(children) => children,
            _ => {
                return Err(
                    "HTML generation error: expected a Document AST root."
                        .to_string()
                );
            }
        };

        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");

        let mut found_head = false;

        for child in children {
            if let AstNode::Head(head_child) = child {
                found_head = true;
                Self::render_children(head_child, &mut html)?;
            }
        }

        if !found_head {
            html.push_str("<title>LOLCODE document</title>\n");
        }

        html.push_str("</head>\n");
        html.push_str("<body>\n");

        for child in children {
            if !matches!(child, AstNode::Head(_)) {
                Self::render_node(child, &mut html)?;
            }
        }

        html.push_str("\n</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    fn render_children(
        children: &[AstNode],
        html: &mut String,
    ) -> Result<(), String> {
        for child in children {
            Self::render_node(child, html)?;
        }

        Ok(())
    }

    fn render_paragraph(
        children: &[AstNode],
        html: &mut String,
    ) -> Result<(), String> {
        let mut paragraph_open = false;
        let mut emitted_content = false;

        for child in children {
            match child {
                AstNode::VariableDefine { .. } => {
                    // Definitions do not produce visible HTML.
                }

                AstNode::List(_) => {
                    if paragraph_open {
                        html.push_str("</p>\n");
                        paragraph_open = false;
                    }

                    Self::render_node(child, html)?;
                    emitted_content = true;
                }

                AstNode::Text(text)
                    if !paragraph_open && text.trim().is_empty() =>
                {
                    // Keep formatting outside a paragraph segment.
                    html.push_str(&Self::escape_html(text));
                }

                _ => {
                    if !paragraph_open {
                        html.push_str("<p>");
                        paragraph_open = true;
                    }

                    Self::render_node(child, html)?;
                    emitted_content = true;
                }
            }
        }

        if paragraph_open {
            html.push_str("</p>\n");
        } else if !emitted_content {
            html.push_str("<p></p>\n");
        }

        Ok(())
    }

    fn render_node(
        node: &AstNode,
        html: &mut String,
    ) -> Result<(), String> {
        match node {
            AstNode::Paragraph(children) => {
                Self::render_paragraph(children, html)?;
            }

            AstNode::List(children) => {
                html.push_str("<ul>\n");
                Self::render_children(children, html)?;
                html.push_str("</ul>\n");
            }

            AstNode::Item(children) => {
                html.push_str("<li>");
                Self::render_children(children, html)?;
                html.push_str("</li>\n");
            }

            AstNode::Title(text) => {
                html.push_str("<title>");
                html.push_str(&Self::escape_html(text));
                html.push_str("</title>\n");
            }

            AstNode::Comment(text) => {
                html.push_str("<!--");
                html.push_str(text);
                html.push_str("-->");
            }

            AstNode::Bold(text) => {
                html.push_str("<b>");
                html.push_str(&Self::escape_html(text));
                html.push_str("</b>");
            }

            AstNode::Text(text) => {
                html.push_str(&Self::escape_html(text));
            }

            AstNode::Link(address) => {
                let escaped = Self::escape_html(address);

                html.push_str("<a href=\"");
                html.push_str(&escaped);
                html.push_str("\">");
                html.push_str(&escaped);
                html.push_str("</a>");
            }

            AstNode::Newline => {
                html.push_str("<br>\n");
            }

            AstNode::VariableDefine { .. } => {
                // Already used during scope resolution.
            }

            AstNode::VariableUse { name } => {
                return Err(format!(
                    "HTML generation error: unresolved variable '{}'.",
                    name
                ));
            }

            AstNode::Document(_) | AstNode::Head(_) => {
                return Err(
                    "HTML generation error: unexpected nested \
                     Document or Head node."
                        .to_string()
                );
            }
        }

        Ok(())
    }
    
}

//              Phase 3
// The final program should take only one command-line argument provided by the user -
// An input file name in the LOLCODE Markdown language

//              Phase 4
// Come up with test input files

//             Phase 5
// Document your code - already on it _( * *)__/ (took me a second to figure out how to do that lol, wanted to make it a salute but wtvr)

//             Phase 6
// Create public documentation using rustdoc