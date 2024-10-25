use crate::models::crate_model::Crate;
use serde::Serialize;

#[derive(Serialize)]
pub struct RustProject {
    pub sysroot: String,
    pub sysroot_src: String,
    pub crates: Vec<Crate>,
}
