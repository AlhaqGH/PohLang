// Placeholder for a future lexer implementation.
// Current parser operates on lines/strings directly.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Minimal placeholder to allow future migration
    Ident(String),
    Number(f64),
    String(String),
}
