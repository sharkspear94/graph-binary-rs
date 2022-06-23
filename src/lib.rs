// #![feature(generic_const_exprs)]
#[macro_use]
extern crate lazy_static;

pub mod de;
pub mod graph_binary;
pub mod ser;
mod specs;

mod client;
mod error;
mod macros;
pub mod message;
pub mod process;
pub mod structure;
#[cfg(test)]
mod tests {}

// Example usage
// fn main() {
// ClientBuilder::new("ws://localhost:8182/gremlin")
//     .unwrap()
//     .connect_insecure()
//     .unwrap();
// }
