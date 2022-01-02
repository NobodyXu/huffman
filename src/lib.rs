pub mod compress;
pub mod encoding;
pub mod tree;

pub const COUNTERS_SIZE: usize = u8::MAX as usize;

pub use compress::{compress, generate_encodings, NonEmptySlice};
pub use encoding::Encoding;
