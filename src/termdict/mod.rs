/*!
The term dictionary contains all of the terms in
`tantivy index` in a sorted manner.

It is implemented as a wrapper of the `Fst` crate in order
to add a value type.

A finite state transducer itself associates
each term `&[u8]` to a `u64` that is in fact an address
in a buffer. The value is then accessible via
deserializing the value at this address.

Keys (`&[u8]`) in this datastructure are sorted.
*/

mod termdict;
mod streamer;
mod merger;

pub use self::termdict::TermDictionary;
pub use self::termdict::TermDictionaryBuilder;
pub use self::streamer::TermStreamer;
pub use self::streamer::TermStreamerBuilder;
pub use self::merger::TermMerger;
