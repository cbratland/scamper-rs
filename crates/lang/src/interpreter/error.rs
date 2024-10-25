use crate::ast::Span;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    // pub code // TODO: errors should have codes
    pub span: Option<Span>,
}

impl RuntimeError {
    pub fn new(message: String, span: Option<Span>) -> Self {
        Self { message, span }
    }

    pub fn emit(&self, src: &str) {
        let line_span = if let Some(span) = self.span {
            let loc = span.loc as usize;
            if loc >= src.len() {
                eprintln!("Error: {}", self.message);
                return;
            }
            let source_up_to = &src[..loc];

            let start_line = bytecount::count(source_up_to.as_bytes(), b'\n') + 1;
            let start_col = loc - source_up_to.rfind('\n').unwrap_or(0);

            let end_loc = loc + (span.len as usize);
            let end_line = start_line + bytecount::count(&src[end_loc..].as_bytes(), b'\n');
            let end_col = end_loc - src[end_loc..].rfind('\n').unwrap_or(0);

            // eprintln!("{line}:{col}");

            // Print the line containing the error
            // let line_start = source_up_to.rfind('\n').map_or(0, |pos| pos + 1);
            // let line_end = src[loc..].find('\n').map_or(src.len(), |pos| loc + pos);
            // let line_content = &src[line_start..line_end];

            // eprintln!("{:2} | {}", line, line_content);
            // eprintln!("   | {}{}", " ".repeat(col), "^".repeat(span.len as usize));

            Some(format!(
                "{}:{}-{}:{}",
                start_line, start_col, end_line, end_col
            ))
        } else {
            None
        };

        eprintln!(
            "Runtime error{}: {}",
            if let Some(line_span) = line_span {
                format!(" [{}]", line_span)
            } else {
                String::default()
            },
            self.message
        );
    }
}
