use super::{borrow_cell::BorrowCell, Encoding, COUNTERS_SIZE};

use binary_heap_plus::BinaryHeap;

#[derive(Debug)]
struct Node {
    cnt: usize,

    parent: u16,

    bit: bool,

    /// the counter of the left is <= counter of the right
    right: u16,
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
                right: u16::MAX,
            });
        }

        let nodes = BorrowCell::new(nodes);

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

                let mut this = Self(inner);
                this.assign_bits();
                break this;
            };

            let parent = {
                let mut nodes = nodes.borrow();

                let cnt = nodes[left as usize].cnt + nodes[right as usize].cnt;

                // Add new parent
                nodes.push(Node {
                    cnt,

                    bit: false,

                    parent: u16::MAX,
                    right,
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

    fn assign_bits(&mut self) {
        let nodes = &mut self.0;

        // Iterate over all internal nodes to assign bits
        for i in 256..511 {
            let right = &mut nodes[nodes[i].right as usize];

            // right.bit should be initialized to `false`.
            //
            // If it is set to `true`, then it means the same node
            // is set to be children of two different nodes.
            debug_assert!(!right.bit);

            right.bit = true;
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
