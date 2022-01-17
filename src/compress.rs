use super::tree::HuffmanTree;
use crate::{Encoding, COUNTERS_SIZE};

use std::ops::Deref;

use bitvec::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct NonEmptySlice<'a>(&'a [u8]);

impl<'a> NonEmptySlice<'a> {
    pub const fn new(slice: &'a [u8]) -> Option<Self> {
        if slice.is_empty() {
            None
        } else {
            Some(Self(slice))
        }
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }
}

// TODO: Fallback to sequential mode if the input is small

/// Return the length of the compressed data in bits and encodings.
pub fn generate_encodings(bytes: NonEmptySlice) -> (usize, Box<[Encoding; COUNTERS_SIZE]>) {
    let counters = bytes
        .into_inner()
        .into_par_iter()
        .fold(
            || {
                let counters: Vec<_> = (0..u8::MAX).map(|_| 0).collect();
                counters.into_boxed_slice()
            },
            |mut counters, byte| {
                counters[*byte as usize] += 1;
                counters
            },
        )
        .reduce_with(|mut counters0, counters1| {
            for (counter0, counter1) in counters0.iter_mut().zip(counters1.iter()) {
                *counter0 += *counter1;
            }

            counters0
        })
        .unwrap();

    let encodings = HuffmanTree::new(counters.deref().try_into().unwrap()).generate_encodings();

    let bits_required: usize = counters
        .iter()
        .zip(encodings.iter())
        .map(|(counter, encoding)| (encoding.len() as usize) * (*counter))
        .sum();

    (bits_required, encodings)
}

pub fn compress(bytes: NonEmptySlice) -> BitBox<Lsb0, u8> {
    let (bits_required, encodings) = generate_encodings(bytes);

    let bitvecs: Vec<_> = bytes
        .into_inner()
        .into_par_iter()
        .map(|byte| encodings[*byte as usize])
        .fold(BitVec::new, |mut bitvec, encoding| {
            bitvec.extend_from_bitslice(encoding.deref());
            bitvec
        })
        .collect();

    bitvecs
        .into_iter()
        .reduce(|mut bitvec0, bitvec1| {
            bitvec0.reserve_exact(bits_required - bitvec0.len());
            bitvec0.extend_from_bitslice(&bitvec1);
            bitvec0
        })
        .unwrap()
        .into_boxed_bitslice()
}
