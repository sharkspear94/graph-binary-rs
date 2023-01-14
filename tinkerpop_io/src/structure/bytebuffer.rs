use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteBuffer(pub(crate) Vec<u8>);

impl ByteBuffer {
    #[must_use]
    pub fn new(buf: &[u8]) -> Self {
        ByteBuffer(buf.to_vec())
    }
    #[must_use]
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }
    #[must_use]
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Display for ByteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{i},")?;
        }
        write!(f, "]")
    }
}
