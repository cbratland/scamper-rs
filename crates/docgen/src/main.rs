use clap::Parser;
use scamper_doc::ScamperDoc;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::{punctuated::Punctuated, visit::Visit, FnArg, Ident, Pat, Token};

fn generate_doc_string(
    doc: &ScamperDoc,
    fn_name: &Ident,
    fn_args: &Punctuated<FnArg, Token![,]>,
) -> (String, String) {
    let mut result = String::new();

    let fn_name_str = fn_name.to_string();

    let name = doc
        .name
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or_else(|| fn_name_str.as_str());
    result.push_str(&format!("({}", name));

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                result.push_str(&format!(" {}", pat_ident.ident));
            }
        }
    }

    if let Some(ret_type) = &doc.return_type {
        result.push_str(&format!(") -> {}\n", ret_type));
    } else {
        result.push_str(")\n");
    }

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let param_name = pat_ident.ident.to_string();
                if let Some(param_doc) = doc.params.iter().find(|p| p.name == param_name) {
                    result.push_str(&format!(
                        "  {}: {}",
                        param_name,
                        param_doc.type_name.as_ref().unwrap_or(&"any".to_string())
                    ));
                    if let Some(desc) = &param_doc.description {
                        result.push_str(&format!(", {}", desc));
                    }
                    result.push('\n');
                }
            }
        }
    }

    (result, name.to_string())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input directory containing Rust source files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for markdown files
    #[arg(short, long)]
    output: PathBuf,
}

struct DocVisitor {
    current_file: PathBuf,
    output_dir: PathBuf,
    current_docs: String,
}

impl DocVisitor {
    fn new(output_dir: PathBuf) -> Self {
        Self {
            current_file: PathBuf::new(),
            output_dir,
            current_docs: String::new(),
        }
    }

    fn write_docs(&mut self) -> std::io::Result<()> {
        if !self.current_docs.is_empty() {
            let file_name = self.current_file.file_stem().unwrap().to_str().unwrap();
            let output_path = self.output_dir.join(format!("{}.md", file_name));
            let mut file = File::create(output_path)?;
            file.write_all(self.current_docs.as_bytes())?;
            self.current_docs.clear();
        }
        Ok(())
    }
}

impl<'ast> Visit<'ast> for DocVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        for attr in &node.attrs {
            if attr.path().is_ident("scamper_doc") {
                if let Ok(doc) = attr.parse_args::<ScamperDoc>() {
                    let (doc_string, name) =
                        generate_doc_string(&doc, &node.sig.ident, &node.sig.inputs);
                    self.current_docs.push_str(&format!(
                        "# {}\n~~~\n{}~~~\n{}\n\n",
                        name, doc_string, doc.description
                    ));
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // create output directory if it doesn't exist
    fs::create_dir_all(&cli.output)?;

    let mut visitor = DocVisitor::new(cli.output);
    visit_rust_files(&cli.input, &mut visitor)?;

    Ok(())
}

fn visit_rust_files(dir: &Path, visitor: &mut DocVisitor) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_rust_files(&path, visitor)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            visitor.current_file = path.clone();
            let content = fs::read_to_string(&path)?;
            let syntax = syn::parse_file(&content).expect("Failed to parse Rust file");
            visitor.visit_file(&syntax);
            visitor.write_docs()?;
        }
    }
    Ok(())
}
