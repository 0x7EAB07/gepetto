/// Core functionality for the Gepetto CLI
///
/// This library provides the underlying functionality that the CLI application uses.

/// Add two numbers together
///
/// # Examples
///
/// ```
/// use gepetto_core::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Multiply two numbers
///
/// # Examples
///
/// ```
/// use gepetto_core::multiply;
///
/// assert_eq!(multiply(2, 3), 6);
/// ```
pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

/// Calculate the factorial of a number
///
/// # Examples
///
/// ```
/// use gepetto_core::factorial;
///
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Generate a greeting message
///
/// # Examples
///
/// ```
/// use gepetto_core::greet;
///
/// assert_eq!(greet("Alice"), "Hello, Alice! Welcome to Gepetto!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Gepetto!", name)
}

/// Process text and return statistics
///
/// # Examples
///
/// ```
/// use gepetto_core::{TextStats, analyze_text};
///
/// let stats = analyze_text("Hello world");
/// assert_eq!(stats.char_count, 11);
/// assert_eq!(stats.word_count, 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TextStats {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
}

pub fn analyze_text(text: &str) -> TextStats {
    TextStats {
        char_count: text.chars().count(),
        word_count: text.split_whitespace().count(),
        line_count: text.lines().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(0, 5), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice! Welcome to Gepetto!");
    }

    #[test]
    fn test_analyze_text() {
        let stats = analyze_text("Hello world\nThis is a test");
        assert_eq!(stats.char_count, 26);
        assert_eq!(stats.word_count, 6);
        assert_eq!(stats.line_count, 2);
    }
}
