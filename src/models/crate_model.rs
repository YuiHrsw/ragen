use serde::Serialize;

#[derive(Serialize)]
pub struct Crate {
    pub root_module: String,
    pub edition: String,
    pub deps: Vec<String>,
}
