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

fn open_in_browser(path: &Path) -> Result<(), String> {
    let absolute_path = fs::canonicalize(path)
        .map_err(|error| {
            format!(
                "Could not locate generated HTML file: {}",
                error
            )
        })?;

    let status = process::Command::new("open")
        .arg(&absolute_path)
        .status()
        .map_err(|error| {
            format!(
                "Could not launch the browser-opening command: {}",
                error
            )
        })?;

    if !status.success() {
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
#[derive(Debug, PartialEq)]
pub enum AstNode {
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

    VariableDefine {
        name: String,
        value: String,
    },

    VariableUse {
        name: String,
    },
}


pub struct Parser {
    tokens: Vec<String>,
    position: usize,
    nodes: Vec<AstNode>, // Added a field to hold the nodes of the abstract syntax tree
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            tokens,
            position: 0,
            nodes: Vec::new(), // Added this - Initializes the nodes vector to hold the abstract syntax tree
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

        let start = self.nodes.len(); // Remember the starting index of the list item

        self.parse_optional_variable_define()?;
        self.parse_inner_list()?;

        self.expect("#OIC")?;
        self.finish_container(start, AstNode::Item);
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

    fn take_plain_text(&mut self) -> Result<String, String> {
        match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                let text = token.to_string();
                self.position += 1;
                Ok(text)
            }

            _ => Err(self.syntax_error("plain text")),
        }
    }

    fn finish_container(
        &mut self,
        start: usize,
        make_node: fn(Vec<AstNode>) -> AstNode,
    ) {
        let children = self.nodes.split_off(start);
        self.nodes.push(make_node(children));
    }
}

impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;

        let start = self.nodes.len();

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

        self.finish_container(start, AstNode::Document);

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        let begins_head =
            self.current_token() == Some("#MAEK")
                && self.next_token_is("HEAD");

        if begins_head {
            self.expect("#MAEK")?;
            self.expect("HEAD")?;

            let start = self.nodes.len();

            self.skip_formatting_whitespace();

            self.parse_title()?;

            self.skip_formatting_whitespace();

            self.expect("#MKAY")?;

            self.finish_container(start, AstNode::Head);
        }

        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("TITLE")?;

        let text = self.take_plain_text()?;

        self.expect("#OIC")?;

        self.nodes.push(AstNode::Title(text));

        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;

        let text = self.take_plain_text()?;

        self.expect("#TLDR")?;

        self.nodes.push(AstNode::Comment(text));

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

        let start = self.nodes.len();

        self.parse_optional_variable_define()?;

        self.parse_inner_paragraph()?;

        self.expect("#MKAY")?;

        self.finish_container(start, AstNode::Paragraph);

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

        let name_position = self.position;

        self.parse_variable_name()?;

        let name = self.tokens[name_position]
            .trim()
            .to_string();

        self.expect("#ITIZ")?;

        let value = self.take_plain_text()?;

        self.expect("#MKAY")?;

        self.nodes.push(AstNode::VariableDefine {
            name,
            value,
        });

        Ok(())
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("VAR")?;

        let name_position = self.position;

        self.parse_variable_name()?;

        let name = self.tokens[name_position]
            .trim()
            .to_string();

        self.expect("#OIC")?;

        self.nodes.push(AstNode::VariableUse {
            name,
        });

        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("BOLD")?;

        let text = self.take_plain_text()?;

        self.expect("#OIC")?;

        self.nodes.push(AstNode::Bold(text));

        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("#MAEK")?;
        self.expect("LIST")?;

        let start = self.nodes.len();

        self.skip_formatting_whitespace();

        self.parse_list_item()?;

        self.parse_list_items()?;

        self.expect("#MKAY")?;

        self.finish_container(start, AstNode::List);

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

        let address = match self.current_token() {
            Some(token) if !token.starts_with('#') => {
                let address = token.trim();

                if address.is_empty()
                    || address.chars().any(|c| c.is_whitespace())
                {
                    return Err(self.syntax_error(
                        "a nonempty address without internal whitespace"
                    ));
                }

                address.to_string()
            }

            _ => {
                return Err(self.syntax_error("a link address"));
            }
        };

        self.position += 1;

        self.expect("#OIC")?;

        self.nodes.push(AstNode::Link(address));

        Ok(())
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")?;

        self.nodes.push(AstNode::Newline);

        Ok(())
    }

    fn parse_text(&mut self) -> Result<(), String> {
        let text = self.take_plain_text()?;

        self.nodes.push(AstNode::Text(text));

        Ok(())
    }
}


//              Phase 2 - Task #3
// Implement the static scope variable resolution for the LOLCODE Markdown language
// Any static semantic errors - variable being used before it is defined -
// encountered should be reported as output to the console with as much error information as possible
// My compiler should exit after the first error is encountered. If an error is encountered, no output file should be created.
pub struct ScopeResolver {
    scopes: Vec<Vec<(String, String)>>, // Stack of scopes, each scope is a vector of (variable name, variable value) pairs
}

impl ScopeResolver {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, tree: &mut AstNode) -> Result<(), String> {
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

impl HtmlGenerator {
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