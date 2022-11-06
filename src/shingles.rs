//! Implements an iterator of shingles over a given slice.
//!
//! # Example
//!
//! ```
//! use aabel_rs::shingles::*;
//!
//! let source = vec![1, 2, 3];
//! let pred = |_: &i32| true;
//! let mut ss = shingles(source.as_slice(), 2, pred);
//! assert_eq!(Some([1, 2].as_slice()), ss.next());
//! assert_eq!(Some([2, 3].as_slice()), ss.next());
//! assert_eq!(None, ss.next());
//! ```

use std::num::NonZeroUsize;

pub struct Shingles<'a, T, P> {
    slice: &'a [T],
    size: NonZeroUsize,
    is_start: P,
}

pub fn shingles<'a, T, P>(slice: &'a [T], size: usize, is_start: P) -> Shingles<'a, T, P> {
    Shingles {
        slice,
        size: NonZeroUsize::new(size).expect("size is zero"),
        is_start,
    }
}

impl<'a, T, P> Iterator for Shingles<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.size.get() > self.slice.len() {
            None
        } else {
            if (self.is_start)(&self.slice[0]) {
                let ret = Some(&self.slice[..self.size.get()]);
                self.slice = &self.slice[1..];
                ret
            } else {
                self.slice = &self.slice[1..];
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shingles_all_() {
        let source = vec![1, 2, 3];
        let pred = |_: &i32| true;

        let mut ss = shingles(source.as_slice(), 2, pred);

        assert_eq!(Some([1, 2].as_slice()), ss.next());
        assert_eq!(Some([2, 3].as_slice()), ss.next());
        assert_eq!(None, ss.next());
    }

    // gets the word-shingles from a given text. The shingles can start only from a stop-word and they have a length of 3.
    #[test]
    fn singles_words_() {
        const SHINGLE_LENGTH: usize = 3;
        let text = "A spokeperson for the Sudzo Corporation \
        revealed today that studies have shown it is good for people \
        to buy Sudzo products"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let stop_words = ["A", "for", "the", "to", "that"].as_slice();
        let is_stop_word = |w: &&str| stop_words.contains(w);

        let mut ss = shingles(text.as_slice(), SHINGLE_LENGTH, is_stop_word);
        assert_eq!(Some(["A", "spokeperson", "for"].as_slice()), ss.next());
        assert_eq!(Some(["for", "the", "Sudzo"].as_slice()), ss.next());
        assert_eq!(Some(["the", "Sudzo", "Corporation"].as_slice()), ss.next());
    }
}
