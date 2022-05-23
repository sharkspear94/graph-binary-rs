mod de;
mod graph_binary;
mod ser;
mod specs;
mod traits;

mod error;
mod macros;
mod message;
mod structure;
#[cfg(test)]
mod tests {}

// Example usage
// fn main() {
//     let int = 15_i32;
//     let option_int = Some(15_i32);

//     let mut buf = [0_u8;256];
//     graph_binary::encode_to_slice(buf,int);

//     let vec: Result<Vec<u8>> = graph_binary::encode_to_vec(int);
//     let vec: Result<Vec<u8>> = graph_binary::encode_to_vec(int);

//     let mut file = File::open(path)?;

//     graph_binary::encode_to_writer(file,int);
// }
