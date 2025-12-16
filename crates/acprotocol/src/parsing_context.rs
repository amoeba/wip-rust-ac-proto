use std::fmt;

/// Severity level of a parsing error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Fatal,
}

/// A parsing error with context information
#[derive(Debug, Clone)]
pub struct ParseError {
    pub severity: ErrorSeverity,
    pub packet_num: Option<usize>,
    pub offset: Option<usize>,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let location = match (self.packet_num, self.offset) {
            (Some(pkt), Some(off)) => format!("Packet {}, offset {}: ", pkt, off),
            (Some(pkt), None) => format!("Packet {}: ", pkt),
            (None, Some(off)) => format!("Offset {}: ", off),
            (None, None) => String::new(),
        };
        write!(f, "{}{:?}: {}", location, self.severity, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Tracks parsing errors and warnings
#[derive(Debug, Clone)]
pub struct ParsingContext {
    errors: Vec<ParseError>,
    current_packet: Option<usize>,
    max_errors: Option<usize>,
    stop_on_fatal: bool,
}

impl ParsingContext {
    /// Create a new parsing context
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            current_packet: None,
            max_errors: None,
            stop_on_fatal: true,
        }
    }

    /// Set the maximum number of errors to track before stopping (None = unlimited)
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = Some(max);
        self
    }

    /// Set whether to stop parsing on fatal errors
    pub fn with_stop_on_fatal(mut self, stop: bool) -> Self {
        self.stop_on_fatal = stop;
        self
    }

    /// Set the current packet number for context
    pub fn set_packet_number(&mut self, packet_num: usize) {
        self.current_packet = Some(packet_num);
    }

    /// Record a warning
    pub fn warn(&mut self, message: impl Into<String>, offset: Option<usize>) {
        self.record_error(ErrorSeverity::Warning, message.into(), offset);
    }

    /// Record an error
    pub fn error(&mut self, message: impl Into<String>, offset: Option<usize>) {
        self.record_error(ErrorSeverity::Error, message.into(), offset);
    }

    /// Record a fatal error
    pub fn fatal(&mut self, message: impl Into<String>, offset: Option<usize>) {
        self.record_error(ErrorSeverity::Fatal, message.into(), offset);
    }

    fn record_error(&mut self, severity: ErrorSeverity, message: String, offset: Option<usize>) {
        if let Some(max) = self.max_errors {
            if self.errors.len() >= max {
                return;
            }
        }

        self.errors.push(ParseError {
            severity,
            packet_num: self.current_packet,
            offset,
            message,
        });
    }

    /// Check if we should continue parsing (stops on fatal if configured)
    pub fn should_continue(&self) -> bool {
        if !self.stop_on_fatal {
            return true;
        }
        !self.errors.iter().any(|e| e.severity == ErrorSeverity::Fatal)
    }

    /// Get all errors
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Get error count by severity
    pub fn error_counts(&self) -> (usize, usize, usize) {
        let mut warnings = 0;
        let mut errors = 0;
        let mut fatals = 0;

        for err in &self.errors {
            match err.severity {
                ErrorSeverity::Warning => warnings += 1,
                ErrorSeverity::Error => errors += 1,
                ErrorSeverity::Fatal => fatals += 1,
            }
        }

        (warnings, errors, fatals)
    }

    /// Print a summary of all errors
    pub fn print_summary(&self) {
        let (warnings, errors, fatals) = self.error_counts();
        let total = warnings + errors + fatals;

        if total == 0 {
            println!("✓ No parsing errors");
            return;
        }

        println!("\n{} parsing issues:", total);
        if warnings > 0 {
            println!("  {} warnings", warnings);
        }
        if errors > 0 {
            println!("  {} errors", errors);
        }
        if fatals > 0 {
            println!("  {} fatal errors", fatals);
        }
    }

    /// Print detailed error report
    pub fn print_errors(&self, limit: Option<usize>) {
        if self.errors.is_empty() {
            println!("✓ No parsing errors");
            return;
        }

        let limit = limit.unwrap_or(self.errors.len());
        println!("\nParsing errors (showing {}/{}):", limit.min(self.errors.len()), self.errors.len());

        for (i, err) in self.errors.iter().enumerate() {
            if i >= limit {
                println!("  ... and {} more", self.errors.len() - limit);
                break;
            }
            println!("  {}", err);
        }
    }
}

impl Default for ParsingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_context() {
        let mut ctx = ParsingContext::new();
        ctx.set_packet_number(1);
        ctx.warn("test warning", Some(42));
        ctx.error("test error", Some(43));

        let (w, e, f) = ctx.error_counts();
        assert_eq!(w, 1);
        assert_eq!(e, 1);
        assert_eq!(f, 0);
    }

    #[test]
    fn test_max_errors() {
        let mut ctx = ParsingContext::new().with_max_errors(2);
        ctx.error("error 1", None);
        ctx.error("error 2", None);
        ctx.error("error 3", None);

        assert_eq!(ctx.errors().len(), 2);
    }
}
