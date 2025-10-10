/// Error handling infrastructure for PohLang
/// Provides structured error types, stack traces, and error operations
use std::fmt;

/// A PohLang error with type, message, and stack trace
#[derive(Clone, Debug)]
pub struct PohError {
    pub kind: ErrorKind,
    pub message: String,
    pub stack_trace: Vec<StackFrame>,
}

/// Error type categories for typed error handling
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// General runtime errors (index out of bounds, undefined variable, etc.)
    RuntimeError,
    /// Type mismatches (adding string to number, etc.)
    TypeError,
    /// Mathematical errors (division by zero, etc.)
    MathError,
    /// File system errors (file not found, permission denied, etc.)
    FileError,
    /// JSON parsing/manipulation errors
    JsonError,
    /// Network/HTTP errors (for future HTTP implementation)
    NetworkError,
    /// User validation errors
    ValidationError,
    /// Custom user-defined error types
    Custom(String),
}

/// A single frame in the call stack for error tracing
#[derive(Clone, Debug)]
pub struct StackFrame {
    pub function: String,
    pub file: String,
    pub line: usize,
}

impl PohError {
    /// Create a new error with the given type and message
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        PohError {
            kind,
            message: message.into(),
            stack_trace: Vec::new(),
        }
    }

    /// Create a new error with a stack trace
    pub fn with_stack_trace(
        kind: ErrorKind,
        message: impl Into<String>,
        stack_trace: Vec<StackFrame>,
    ) -> Self {
        PohError {
            kind,
            message: message.into(),
            stack_trace,
        }
    }

    /// Get the error type as a human-readable string
    pub fn type_string(&self) -> String {
        match &self.kind {
            ErrorKind::RuntimeError => "RuntimeError".to_string(),
            ErrorKind::TypeError => "TypeError".to_string(),
            ErrorKind::MathError => "MathError".to_string(),
            ErrorKind::FileError => "FileError".to_string(),
            ErrorKind::JsonError => "JsonError".to_string(),
            ErrorKind::NetworkError => "NetworkError".to_string(),
            ErrorKind::ValidationError => "ValidationError".to_string(),
            ErrorKind::Custom(name) => name.clone(),
        }
    }

    /// Get the error type as a natural English description
    pub fn type_description(&self) -> &str {
        match &self.kind {
            ErrorKind::RuntimeError => "a runtime error",
            ErrorKind::TypeError => "a type error",
            ErrorKind::MathError => "a math error",
            ErrorKind::FileError => "a file error",
            ErrorKind::JsonError => "a JSON error",
            ErrorKind::NetworkError => "a network error",
            ErrorKind::ValidationError => "a validation error",
            ErrorKind::Custom(_) => "an error",
        }
    }

    /// Check if this error matches the given type string
    pub fn matches_type(&self, type_str: &str) -> bool {
        self.type_string().eq_ignore_ascii_case(type_str)
    }

    /// Format the error with stack trace for display
    pub fn format_with_trace(&self) -> String {
        // Natural English format with type marker for parsing
        // Format: [ErrorType] Natural message
        let type_marker = format!("[{}]", self.type_string());
        let mut output = if matches!(self.kind, ErrorKind::Custom(_)) {
            // For custom errors, show the type name
            format!(
                "{} {} occurred: {}",
                type_marker,
                self.type_string(),
                self.message
            )
        } else {
            // For built-in errors, use natural description
            format!(
                "{} Error occurred: {} - {}",
                type_marker,
                self.type_description(),
                self.message
            )
        };

        if !self.stack_trace.is_empty() {
            output.push_str("\nCall stack:");
            for frame in &self.stack_trace {
                output.push_str(&format!(
                    "\n  in {} at {}:{}",
                    frame.function, frame.file, frame.line
                ));
            }
        }

        output
    }
}

impl fmt::Display for PohError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_with_trace())
    }
}

impl ErrorKind {
    /// Parse an error type string into an ErrorKind
    pub fn from_string(s: &str) -> Self {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "runtimeerror" => ErrorKind::RuntimeError,
            "typeerror" => ErrorKind::TypeError,
            "matherror" => ErrorKind::MathError,
            "fileerror" => ErrorKind::FileError,
            "jsonerror" => ErrorKind::JsonError,
            "networkerror" => ErrorKind::NetworkError,
            "validationerror" => ErrorKind::ValidationError,
            _ => ErrorKind::Custom(s.to_string()), // Preserve original casing for custom types
        }
    }
}

impl StackFrame {
    /// Create a new stack frame
    pub fn new(function: impl Into<String>, file: impl Into<String>, line: usize) -> Self {
        StackFrame {
            function: function.into(),
            file: file.into(),
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = PohError::new(ErrorKind::MathError, "Division by zero");
        assert_eq!(error.type_string(), "MathError");
        assert_eq!(error.message, "Division by zero");
        assert!(error.stack_trace.is_empty());
    }

    #[test]
    fn test_error_type_matching() {
        let error = PohError::new(ErrorKind::FileError, "File not found");
        assert!(error.matches_type("FileError"));
        assert!(error.matches_type("fileerror")); // case insensitive
        assert!(!error.matches_type("MathError"));
    }

    #[test]
    fn test_custom_error_type() {
        let error = PohError::new(ErrorKind::Custom("MyError".to_string()), "Custom error");
        assert_eq!(error.type_string(), "MyError");
        assert!(error.matches_type("MyError"));
    }

    #[test]
    fn test_error_with_stack_trace() {
        let trace = vec![
            StackFrame::new("main", "main.poh", 10),
            StackFrame::new("calculate", "math.poh", 42),
        ];
        let error = PohError::with_stack_trace(ErrorKind::MathError, "Division by zero", trace);

        assert_eq!(error.stack_trace.len(), 2);
        assert_eq!(error.stack_trace[0].function, "main");
        assert_eq!(error.stack_trace[1].function, "calculate");
    }

    #[test]
    fn test_error_kind_from_string() {
        assert_eq!(ErrorKind::from_string("MathError"), ErrorKind::MathError);
        assert_eq!(ErrorKind::from_string("matherror"), ErrorKind::MathError); // case insensitive
        assert_eq!(ErrorKind::from_string("FileError"), ErrorKind::FileError);
        assert_eq!(
            ErrorKind::from_string("CustomError"),
            ErrorKind::Custom("customerror".to_string())
        );
    }

    #[test]
    fn test_error_format_with_trace() {
        let trace = vec![
            StackFrame::new("main", "main.poh", 10),
            StackFrame::new("process", "lib.poh", 25),
        ];
        let error = PohError::with_stack_trace(ErrorKind::RuntimeError, "Test error", trace);

        let formatted = error.format_with_trace();
        assert!(formatted.contains("RuntimeError: Test error"));
        assert!(formatted.contains("Stack trace:"));
        assert!(formatted.contains("at main (main.poh:10)"));
        assert!(formatted.contains("at process (lib.poh:25)"));
    }

    #[test]
    fn test_error_display() {
        let error = PohError::new(ErrorKind::TypeError, "Type mismatch");
        let display = format!("{}", error);
        assert!(display.contains("TypeError: Type mismatch"));
    }

    #[test]
    fn test_all_error_kinds() {
        let kinds = vec![
            (ErrorKind::RuntimeError, "RuntimeError"),
            (ErrorKind::TypeError, "TypeError"),
            (ErrorKind::MathError, "MathError"),
            (ErrorKind::FileError, "FileError"),
            (ErrorKind::JsonError, "JsonError"),
            (ErrorKind::NetworkError, "NetworkError"),
            (ErrorKind::ValidationError, "ValidationError"),
            (ErrorKind::Custom("Test".to_string()), "Test"),
        ];

        for (kind, expected) in kinds {
            let error = PohError::new(kind, "test");
            assert_eq!(error.type_string(), expected);
        }
    }
}
