use super::codes::ErrorCode;
use crate::ast::Span;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorLevel {
    Warning,
    Error,
    Help,
    Note,
}

#[derive(Debug)]
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
    pub fn emit(&self, file: &str, src: &str) {
        let level = match self.level {
            ErrorLevel::Error => "error",
            ErrorLevel::Warning => "warning",
            ErrorLevel::Help => "help",
            ErrorLevel::Note => "note",
        };

        eprintln!(
            "{level}{}: {}",
            if let Some(code) = self.code {
                format!("[{}]", code)
            } else {
                String::default()
            },
            self.message
        );

        if let Some(span) = self.span {
            let loc = span.loc as usize;
            let source_up_to = &src[..loc];
            let line = bytecount::count(source_up_to.as_bytes(), b'\n') + 1;
            let col = loc - source_up_to.rfind('\n').unwrap_or(0);

            let line_start = source_up_to.rfind('\n').map_or(0, |pos| pos + 1);
            let line_end = src[loc..].find('\n').map_or(src.len(), |pos| loc + pos);
            let line_content = &src[line_start..line_end];

            eprintln!("  --> {}:{line}:{col}", file);
            eprintln!("   |");
            eprintln!("{:2} | {}", line, line_content);
            eprintln!("   | {}{}", " ".repeat(col), "^".repeat(span.len as usize));
        }

        if let Some(help) = &self.help {
            eprintln!("help: {}", help);
        }

        if let Some(note) = &self.note {
            eprintln!("note: {}", note);
        }
    }
}
