use super::{lexer::TokenStream, ParseResult};

/// This is parses common miscellaneous expressions.
/// At the time, that's only generic identifiers, so maybe this module should be renamed.
/// 
/// This is documented in `grammar/general.ebnf`
pub mod general;

/// This contains everything related to imports and the `use` directive;
/// heavily inspired by `rustc_ast::ast::UseTree` 
/// 
/// This is documented in `grammar/general.ebnf`
pub mod use_tree;

/// This parses type expressions, as well as top-level type alias directives.
/// 
/// This is documented in `general/data.ebnf`
pub mod types;

/// This parses interface expressions.
pub mod interfaces;


//
// Parsing interface
//

pub trait Expectable where Self: std::marker::Sized { 
    /// Tries to extract the implementing AST node from the `stream`,
    /// returning an Unexpected error if it cannot.  
    fn parse_expect(stream: &mut TokenStream) -> ParseResult<Self>;
}

/// A `Peekable` is an AST node with an unambiguous initial token.
/// That lets us implement `parse_peek`, which can gracefully backtrack
/// if the stream cannot derive the implementing AST node from its current position.
pub trait Peekable where Self: std::marker::Sized { 
    /// Tries to extract the implementing AST node from the `stream`,
    /// first by having a peek at the first token.
    /// If the first token doesn't match, return `None` 
    fn parse_peek(stream: &mut TokenStream) -> ParseResult<Option<Self>>;
}