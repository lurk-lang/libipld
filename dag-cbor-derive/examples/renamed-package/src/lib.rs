//! The purpose of this example is to test whether the derive compiles if the lurk-ipld package was
//! renamed in the `Cargo.toml` file.
use ipld::DagCbor;

#[derive(Clone, DagCbor, Debug, Default, PartialEq)]
struct NamedStruct {
    boolean: bool,
    integer: u32,
    float: f64,
    string: String,
}
