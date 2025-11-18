//! Tana Validation Library
//!
//! Shared validation and error formatting logic for Tana smart contracts.
//! Supports both native Rust and WebAssembly compilation.

use wasm_bindgen::prelude::*;

/// Format a validation error with beautiful Rust/Gleam-style output
///
/// This function creates consistent error messages across all Tana systems:
/// - tana-runtime (native Rust)
/// - tana-edge (native Rust)
/// - playground (WASM in browser)
/// - CLI tools (WASM in Bun/Node)
///
/// # Arguments
///
/// * `code` - The source code containing the error
/// * `file_path` - Path to the file (e.g., "contract.ts")
/// * `error_kind` - Category of error (e.g., "Invalid Import", "Type Error")
/// * `line_num` - Line number (1-indexed)
/// * `col_num` - Column number (1-indexed)
/// * `message` - Error message
/// * `help` - Help text explaining how to fix
/// * `underline_length` - Number of characters to underline (for ^^^)
///
/// # Example
///
/// ```rust
/// use tana_validation::format_validation_error;
///
/// let error = format_validation_error(
///     "import { console } from 'tana/invalid';",
///     "contract.ts",
///     "Invalid Import",
///     1,
///     26,
///     "Module 'tana/invalid' not found",
///     "Available modules: tana/core, tana/kv, tana/block",
///     12
/// );
///
/// // Produces:
/// // Validation Error
/// // ❌ Invalid Import
/// //
/// // ┌─ contract.ts:1:26
/// // │
/// //   1 │ import { console } from 'tana/invalid';
/// //     │                          ^^^^^^^^^^^^ Module 'tana/invalid' not found
/// // │
/// // = help: Available modules: tana/core, tana/kv, tana/block
/// // │
/// ```
#[wasm_bindgen]
pub fn format_validation_error(
    code: &str,
    file_path: &str,
    error_kind: &str,
    line_num: usize,
    col_num: usize,
    message: &str,
    help: &str,
    underline_length: usize,
) -> String {
    format_error_impl(code, file_path, error_kind, line_num, col_num, message, help, underline_length)
}

/// Internal implementation of error formatting
/// Used by both WASM binding and native Rust code
fn format_error_impl(
    code: &str,
    file_path: &str,
    error_kind: &str,
    line_num: usize,
    col_num: usize,
    message: &str,
    help: &str,
    underline_length: usize,
) -> String {
    // Get the problematic line
    let lines: Vec<&str> = code.lines().collect();
    let error_line = if line_num > 0 && line_num <= lines.len() {
        lines[line_num - 1]
    } else {
        ""
    };

    // Ensure underline length is at least 1
    let underline_length = underline_length.max(1);

    // Build the error message with consistent formatting
    format!(
        "\nValidation Error\n\
        ❌ {}\n\
        \n\
        ┌─ {}:{}:{}\n\
        │\n\
        {:>3} │ {}\n\
            │ {}{} {}\n\
        │\n\
        = help: {}\n\
        │\n",
        error_kind,
        file_path,
        line_num,
        col_num,
        line_num,
        error_line,
        " ".repeat(col_num.saturating_sub(1)),
        "^".repeat(underline_length),
        message,
        help
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_error_formatting() {
        let code = "import { console } from 'tana/invalid';";
        let error = format_validation_error(
            code,
            "test.ts",
            "Invalid Import",
            1,
            26,
            "Module 'tana/invalid' not found",
            "Available modules: tana/core, tana/kv",
            12,
        );

        assert!(error.contains("❌ Invalid Import"));
        assert!(error.contains("test.ts:1:26"));
        assert!(error.contains("tana/invalid"));
        assert!(error.contains("^^^^^^^^^^^^")); // 12 carets
        assert!(error.contains("= help: Available modules"));
    }

    #[test]
    fn test_multiline_code() {
        let code = "line 1\nline 2 with error\nline 3";
        let error = format_validation_error(
            code,
            "multi.ts",
            "Type Error",
            2,
            7,
            "Something wrong here",
            "Fix it like this",
            4,
        );

        assert!(error.contains("❌ Type Error"));
        assert!(error.contains("multi.ts:2:7"));
        assert!(error.contains("line 2 with error"));
        assert!(error.contains("^^^^")); // 4 carets
    }

    #[test]
    fn test_underline_length_minimum() {
        let error = format_validation_error(
            "test",
            "test.ts",
            "Error",
            1,
            1,
            "msg",
            "help",
            0, // Should become 1
        );

        assert!(error.contains("^")); // At least one caret
    }

    #[test]
    fn test_out_of_bounds_line() {
        let error = format_validation_error(
            "only one line",
            "test.ts",
            "Error",
            999,
            1,
            "msg",
            "help",
            5,
        );

        // Should handle gracefully without panicking
        assert!(error.contains("❌ Error"));
        assert!(error.contains("999 │")); // Shows requested line number
    }
}
