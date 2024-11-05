use syn::{bracketed, parse::Parse, parse::ParseStream, Ident, LitStr, Token};

#[derive(Debug)]
pub struct ParamDoc {
    pub name: String,
    pub type_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct ScamperDoc {
    pub name: Option<String>,
    pub description: String,
    pub params: Vec<ParamDoc>,
    pub return_type: Option<String>,
}

// Parse implementation for documentation attributes
impl Parse for ScamperDoc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut params = Vec::new();
        let mut return_type = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => {
                    let value: LitStr = input.parse()?;
                    name = Some(value.value());
                }
                "description" => {
                    let value: LitStr = input.parse()?;
                    description = Some(value.value());
                }
                "param" => {
                    let content;
                    bracketed!(content in input);
                    let name: LitStr = content.parse()?;
                    let type_name = if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                        Some(content.parse::<LitStr>()?.value())
                    } else {
                        None
                    };
                    let description = if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                        Some(content.parse::<LitStr>()?.value())
                    } else {
                        None
                    };
                    params.push(ParamDoc {
                        name: name.value(),
                        type_name,
                        description,
                    });
                }
                "return_type" => {
                    let value: LitStr = input.parse()?;
                    return_type = Some(value.value());
                }
                _ => return Err(input.error("unexpected documentation key")),
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ScamperDoc {
            name,
            description: description.unwrap_or_default(),
            params,
            return_type,
        })
    }
}
