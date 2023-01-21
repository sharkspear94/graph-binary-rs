use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteBuffer(pub(crate) Vec<u8>);

impl ByteBuffer {
    #[must_use]
    pub fn new(buf: Vec<u8>) -> Self {
        ByteBuffer(buf)
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
    pub fn iter<'a>(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }
    pub fn iter_mut<'a>(&mut self) -> std::slice::IterMut<'_, u8> {
        self.0.iter_mut()
    }
}

impl IntoIterator for ByteBuffer {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ByteBuffer {
    type Item = &'a u8;

    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ByteBuffer {
    type Item = &'a mut u8;

    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
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
