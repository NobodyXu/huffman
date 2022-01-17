use std::fmt;
use std::ops::Deref;

use bitvec::prelude::*;

/// The encoding can have at most 255 bits,
/// since the Huffman tree can have at most 256 layers, however
/// the root node does not participate in encoding, thus the
/// maximum length of the symbol is 255 bits.
#[derive(Copy, Clone, Eq)]
pub struct Encoding {
    inner: [u8; 32],
    len: u8,
}

impl Encoding {
    pub const fn new() -> Self {
        Self {
            inner: [0; 32],
            len: 0,
        }
    }

    /// Must not push more than 255 bits.
    pub fn push(&mut self, bit: bool) {
        let bitslice: &mut BitSlice<Lsb0, u8> = BitSlice::from_slice_mut(&mut self.inner).unwrap();
        bitslice.set(self.len as usize, bit);
        self.len.checked_add(1).unwrap();
    }

    pub const fn len(&self) -> u8 {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reverse(&self) -> Self {
        let mut reversed_encoding = Self::new();

        for bit in self.deref().into_iter().rev() {
            reversed_encoding.push(*bit);
        }

        reversed_encoding
    }
}

impl Deref for Encoding {
    type Target = BitSlice<Lsb0, u8>;

    fn deref(&self) -> &Self::Target {
        let bitslice = BitSlice::from_slice(&self.inner).unwrap();
        bitslice.split_at(self.len as usize).0
    }
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl fmt::Debug for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Encoding;
    use bitvec::prelude::*;

    #[test]
    fn test_push() {
        let mut encoding = Encoding::new();
        let mut bitvec = BitVec::<Lsb0, u8>::new();

        assert!(encoding.is_empty());

        for i in 1..=255 {
            encoding.push(true);
            bitvec.push(true);

            assert_eq!(encoding.len(), i);

            assert_eq!(*encoding, *bitvec);
        }
    }
}
