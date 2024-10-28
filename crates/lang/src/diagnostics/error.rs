use super::codes::ErrorCode;
use crate::ast::Span;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ErrorLevel {
    Warning,
    Error,
    Help,
    Note,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub level: ErrorLevel,
    pub code: Option<ErrorCode>,
    pub message: String,
    pub span: Option<Span>,
    pub help: Option<String>,
    pub note: Option<String>,
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, span: Option<Span>) -> Self {
        Self {
            level: ErrorLevel::Error,
            code: None,
            message: message.into(),
            span,
            help: None,
            note: None,
        }
    }

    // pub fn warn<S: Into<String>>(message: S, span: Option<Span>) -> Self {
    //     Self {
    //         level: ErrorLevel::Warning,
    //         code: None,
    //         message: message.into(),
    //         span,
    //         help: None,
    //         note: None,
    //     }
    // }

    pub fn with_code(mut self, code: u32) -> Self {
        self.code = Some(ErrorCode::new(code));
        self
    }

    // pub fn with_help(mut self, help: impl Into<String>) -> Self {
    //     self.help = Some(help.into());
    //     self
    // }

    // pub fn with_note(mut self, note: impl Into<String>) -> Self {
    //     self.note = Some(note.into());
    //     self
    // }
}

impl ParseError {
    pub fn emit_to_string(&self, file: &str, src: &str) -> String {
        let level = match self.level {
            ErrorLevel::Error => "error",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Help => "help",
            ErrorLevel::Note => "note",
        };

        let mut error_lines = vec![format!(
            "{level}{}: {}",
            if let Some(code) = self.code {
                format!("[{}]", code)
            } else {
                String::default()
            },
            self.message
        )];

        if let Some(span) = self.span {
            let loc = span.loc as usize;
            let source_up_to = &src[..loc];
            let line = bytecount::count(source_up_to.as_bytes(), b'\n') + 1;
            let col = loc - source_up_to.rfind('\n').unwrap_or(0);

            let line_start = source_up_to.rfind('\n').map_or(0, |pos| pos + 1);
            let line_end = src[loc..].find('\n').map_or(src.len(), |pos| loc + pos);
            let line_content = &src[line_start..line_end];

            error_lines.push(format!("  --> {}:{line}:{col}", file));
            error_lines.push(format!("   |"));
            error_lines.push(format!("{:2} | {}", line, line_content));
            error_lines.push(format!(
                "   | {}{}",
                " ".repeat(col),
                "^".repeat(span.len as usize)
            ));
        }

        if let Some(help) = &self.help {
            error_lines.push(format!("help: {}", help));
        }

        if let Some(note) = &self.note {
            error_lines.push(format!("note: {}", note));
        }

        error_lines.join("\n")
    }

    // Parser error [23:1-23:1]: Unterminated string literal.
    pub fn emit_to_web_string(&self, src: &str) -> String {
        let line_span = if let Some(span) = self.span {
            let loc = span.loc as usize;
            if loc >= src.len() {
                return format!("Parser error: {}", self.message);
            }

            let end = loc + span.len as usize;

            let source_up_to_start = &src[..loc];
            let source_up_to_end = &src[..end];

            let start_line = bytecount::count(source_up_to_start.as_bytes(), b'\n') + 1;
            let end_line = bytecount::count(source_up_to_end.as_bytes(), b'\n') + 1;

            let start_col = loc - source_up_to_start.rfind('\n').unwrap_or(0);
            let end_col = end - source_up_to_end.rfind('\n').unwrap_or(0) - 1;

            Some(format!(
                "{}:{}-{}:{}",
                start_line, start_col, end_line, end_col
            ))
        } else {
            None
        };

        // capitalize the first letter of the message
        let message = self.message.get(0..1).map_or(String::new(), |f| {
            f.to_uppercase() + &self.message[1..] + "."
        });

        format!(
            "Parser error{}: {}",
            if let Some(line_span) = line_span {
                format!(" [{}]", line_span)
            } else {
                String::default()
            },
            message
        )
    }

    pub fn emit(&self, file: &str, src: &str) {
        eprintln!("{}", self.emit_to_string(file, src));
    }
}
