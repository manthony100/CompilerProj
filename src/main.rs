// Main function
fn main() {
    println!("Hello, world!");
    // Input for the LOLCODE program
    // let lolcodeInput: &str = "..........";

    // Create a new instance of the Compiler struct
    // let mut compiler = Compiler::new(lolcodeInput);
    // Create a new instance of the LexicalAnalyzer struct with the lolcode input
    // let mut lexer: LexicalAnalyzer = LexicalAnalyzer::new(lolcodeInput);

    // Initialize the current token with the first token from the lexer
    // let compiler.current_token = lexer.start();

    // Create a new instance of the SyntaxAnalyzer struct with references to the lexer and compiler
    // let mut syntax_analyzer = SyntaxAnalyzer::new(&lexer, &compiler);
    // Call the lolspeak method to parse the lolcode according to the grammar rules
    // parser.lolspeak();

    // Check if there are any remaining tokens or if the current token is not empty
    // if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
            // Print an error message if there are any additional tokens
            // eprintln!("A syntax error was encountered. Additional tokens found.");
            // Exit the program with an error code
            // std::process::exit(1);
    // }
}

//              Phase 2 - Task #1 

// Character by character lexical analyzer

// Partitions the lexemes of a source file in our LOLCODE Markdown language into tokens

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
/// Return the next character from the input.
/// If input is exhausted, should terminate the program.
fn get_char(&mut self) -> char;
/// Add a character to the current potential token.
fn add_char(&mut self, c: char);
/// Lookup a potential token to determine if it is valid.
/// Returns true if a valid token/lexeme, false otherwise.
fn lookup(&self, s: &str) -> bool;
}

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