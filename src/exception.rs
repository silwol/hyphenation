//! Data structures and methods for parsing and applying exceptions, which
//! assign predetermined scores to specific words.

use std::collections::hash_map::{HashMap};

use klpair::KLPair;


/// A specialized hash map of pattern-score pairs.
#[derive(Clone, Debug)]
pub struct Exceptions(pub HashMap<String, Vec<u32>>);

impl Exceptions {
    /// Creates an empty `Exceptions` map.
    pub fn empty() -> Exceptions {
        Exceptions(HashMap::new())
    }

    /// Inserts a Knuth-Liang exception pair into the map.
    ///
    /// If the pattern already existed, the old score is returned; if not, `None` is.
    pub fn insert(&mut self, klpair: KLPair) -> Option<Vec<u32>> {
        let (p, score) = klpair;
        let Exceptions(ref mut m) = *self;

        m.insert(p, score)
    }

    /// Retrieves the score for each hyphenation point of `word`.
    pub fn score(&self, word: &str) -> Option<&Vec<u32>> {
        let Exceptions(ref m) = *self;
        let w = word.to_lowercase();

        m.get(&w)
    }
}
