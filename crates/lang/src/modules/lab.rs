use crate::interpreter::Env;
use scamper_macros::{function, ForeignValue};

pub fn add_to(env: &mut Env) {
    env.register("title", title);
    env.register("part", part);
    env.register("problem", problem);
    env.register("description", description);
}

#[derive(Debug, Clone, ForeignValue)]
pub enum LabElement {
    Title(String),
    Part(String),
    Problem(String),
    Description(String),
}

#[function]
fn title(text: String) -> LabElement {
    LabElement::Title(text)
}

#[function]
fn part(text: String) -> LabElement {
    LabElement::Part(text)
}

#[function]
fn problem(text: String) -> LabElement {
    LabElement::Problem(text)
}

#[function]
fn description(text: String) -> LabElement {
    LabElement::Description(text)
}
