use crate::ast::Span;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    // pub code // TODO: errors should have codes
    pub span: Option<Span>,
}

impl RuntimeError {
    pub fn new(message: String, span: Option<Span>) -> Self {
        Self { message, span }
    }

    pub fn emit_to_string(&self, src: &str) -> String {
        let line_span = if let Some(span) = self.span {
            let loc = span.loc as usize;
            if loc >= src.len() {
                return format!("Runtime error: {}", self.message);
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

        return format!(
            "Runtime error{}: {}.",
            if let Some(line_span) = line_span {
                format!(" [{}]", line_span)
            } else {
                String::default()
            },
            self.message
        );
    }

    pub fn emit(&self, src: &str) {
        eprintln!("{}", self.emit_to_string(src));
    }
}
