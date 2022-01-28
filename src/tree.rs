use super::{Encoding, COUNTERS_SIZE};

use std::cell::RefCell;

use binary_heap_plus::BinaryHeap;

#[derive(Debug)]
struct Node {
    cnt: usize,
    parent: u16,
    bit: bool,
}

#[derive(Debug)]
pub struct HuffmanTree(Box<[Node]>);

impl HuffmanTree {
    pub fn new(counters: &[usize; COUNTERS_SIZE]) -> Self {
        let mut nodes = Vec::with_capacity(511);
        for cnt in counters.iter().copied() {
            nodes.push(Node {
                cnt,
                bit: false,
                parent: u16::MAX,
            });
        }

        let nodes = RefCell::new(nodes);

        let mut heap = BinaryHeap::from_vec_cmp((0_u16..255_u16).collect(), |x: &u16, y: &u16| {
            let nodes = nodes.borrow();
            let x = &nodes[*x as usize];
            let y = &nodes[*y as usize];

            y.cnt.cmp(&x.cnt)
        });

        loop {
            let left = heap.pop().unwrap();

            let right = if let Some(right) = heap.pop() {
                right
            } else {
                let inner = nodes.into_inner().into_boxed_slice();

                debug_assert_eq!(inner.len(), 511);
                debug_assert_eq!(left, 510);

                break Self(inner);
            };

            let parent = {
                let mut nodes = nodes.borrow_mut();

                let cnt = nodes[left as usize].cnt + nodes[right as usize].cnt;

                // Add new parent
                nodes.push(Node {
                    cnt,
                    bit: false,
                    parent: u16::MAX,
                });

                // Index of the newly added parent
                let parent: u16 = (nodes.len() - 1).try_into().unwrap();

                nodes[left as usize].parent = parent;
                nodes[right as usize].parent = parent;

                parent
            };

            heap.push(parent);
        }
    }

    pub fn generate_encodings(&self) -> Box<[Encoding; COUNTERS_SIZE]> {
        let nodes = &self.0;
        let root_index = (nodes.len() - 1) as u16;

        let encodings: Vec<_> = nodes[0..256]
            .iter()
            .map(|mut node| {
                let mut encoding = Encoding::new();

                loop {
                    encoding.push(node.bit);

                    let parent = node.parent;
                    if parent == root_index {
                        break;
                    }

                    node = &nodes[parent as usize];
                }

                encoding.reverse();
                encoding
            })
            .collect();

        encodings.into_boxed_slice().try_into().unwrap()
    }
}
