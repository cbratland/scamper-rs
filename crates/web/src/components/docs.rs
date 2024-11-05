use lazy_static::lazy_static;
use leptos::*;
use leptos_meta::Stylesheet;
use leptos_router::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Function {
    name: String,
    signature: String,
    description: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Module {
    name: String,
    functions: Vec<Function>,
}

fn parse_markdown(content: &str, module_name: &str) -> Module {
    let blocks: Vec<&str> = content.split("\n\n").collect();
    let mut functions = Vec::new();

    for chunk in blocks {
        if chunk.is_empty() {
            continue;
        }

        let mut lines = chunk.lines();

        let name = lines
            .next()
            .unwrap_or("")
            .trim_start_matches('#')
            .trim()
            .to_string();

        let mut signature = String::new();
        let mut description = String::new();

        let mut in_code_block = false;
        for line in lines {
            if line.starts_with("~~~") {
                in_code_block = !in_code_block;
                continue;
            }

            if in_code_block {
                signature.push_str(line);
                signature.push('\n');
            } else {
                description.push_str(line);
                description.push('\n');
            }
        }

        functions.push(Function {
            name,
            signature,
            description,
        });
    }

    // sort functions alphabetically
    functions.sort_by(|a, b| a.name.cmp(&b.name));

    Module {
        name: module_name.to_string(),
        functions,
    }
}

lazy_static! {
    static ref DOCUMENTATION: HashMap<String, Module> = {
        let mut modules = HashMap::new();

        let files = [
            ("prelude", include_str!("../../build/docs/prelude.md")),
            // ...
        ];

        for (name, content) in files.iter() {
            let module = parse_markdown(content, name);
            modules.insert(name.to_string(), module);
        }

        modules
    };
}

#[component]
fn DocSidebar() -> impl IntoView {
    view! {
        <div id="index">
            <ul>
                {DOCUMENTATION.iter().map(|(name, _)| {
                    view! {
                        <li>
                            <A href={format!("/docs/{}", name)}>
                                {name.to_string()}
                            </A>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn FunctionSidebar(module: Module) -> impl IntoView {
    view! {
        <div id="function-index">
            <strong>{module.name.clone()}</strong>
            <ul>
                {module.functions.iter().map(|func| {
                    view! {
                        <li>
                            <a href={format!("#{}-{}", module.name, func.name)}>
                                {func.name.clone()}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

// replace ` with <code>, * with <em>, and ** with <strong>
fn basic_md_to_html(str: &str) -> String {
    let mut result = String::new();
    let mut in_code = false;
    let mut in_em = false;
    let mut in_strong = false;
    let mut chars = str.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '`' => {
                if in_code {
                    result.push_str("</code>");
                } else {
                    result.push_str("<code>");
                }
                in_code = !in_code;
            }
            '*' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '*' {
                        chars.next();
                        if in_strong {
                            result.push_str("</strong>");
                        } else {
                            result.push_str("<strong>");
                        }
                        in_strong = !in_strong;
                    } else {
                        if in_em {
                            result.push_str("</em>");
                        } else {
                            result.push_str("<em>");
                        }
                        in_em = !in_em;
                    }
                } else {
                    if in_em {
                        result.push_str("</em>");
                    } else {
                        result.push_str("<em>");
                    }
                    in_em = !in_em;
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

#[component]
fn ModuleContent(module: Module) -> impl IntoView {
    view! {
        <div id="docs">
            <For
                each=move || module.functions.clone()
                key=|function| function.signature.clone()
                let:func
            >
                <div id=format!("{}-{}", module.name, func.name) class="entry">
                    <pre><code>{func.signature}</code></pre>
                    <p inner_html={basic_md_to_html(&func.description)} />
                </div>
            </For>
        </div>
    }
}

#[component]
pub fn Docs() -> impl IntoView {
    let params = use_params_map();
    let current_module = create_memo(move |_| {
        params.with(|params| {
            params
                .get("module")
                .and_then(|module| DOCUMENTATION.get(module))
                .cloned()
        })
    });

    view! {
        <Stylesheet href="../docs.css"/>
        <div id="content">
            <DocSidebar/>
            <div id="module-content">
                {move || match current_module.get() {
                    Some(module) => view! {
                        <FunctionSidebar module=module.clone()/>
                        <ModuleContent module=module/>
                    }.into_view(),
                    None => view! {
                        <div id="select">
                            "Select a module from the sidebar"
                        </div>
                    }.into_view(),
                }}
            </div>
        </div>
    }
}
