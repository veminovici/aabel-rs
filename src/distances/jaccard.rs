use std::hash::Hash;

use crate::collections::{CountedMap, FromKeys, FromKeysAndValues};

/// A structure which contains the components
/// for the Jaccard similarity between two collections.
/// 
/// The Jaccard instance can be constructed from collections
/// of key-value pairs, using the [`from_pairs`](Jaccard::from_pairs) constructor function.
/// A second way to construct the Jaccard instance is to use
/// collections of keys, using the [`from_keys`](Jaccard::from_keys) construction function.
/// 
/// The Jaccard similarity value can be obtain converting the Jaccard instance to [`f32`].
/// 
/// ## Examples
/// 
/// An example for computing the Jaccard similarity for two collections represented
/// as key-value pairs.
/// 
/// ```
/// use aabel::distances::*;
/// 
/// let xs = [("a", 3), ("b", 1)];
/// let ys = [("a", 2), ("b", 2), ("c", 1)];
/// let j: f32 = Jaccard::from_pairs(xs).and_with(ys).compute().into();
/// assert_eq!(1 as f32 / 3 as f32, j);
/// ```
/// 
/// An example for computing the Jaccard similarity for two collections represented
/// as keys.
/// 
/// ```
/// use aabel::distances::*;
/// 
/// let xs = ["a", "a", "b", "a"];
/// let ys = ["a", "b", "b", "a", "c"];
/// let j: f32 = Jaccard::from_keys(xs).and_with(ys).compute().into();
/// assert_eq!(1 as f32 / 3 as f32, j);
/// ```
pub struct Jaccard {
    cmn: i32,
    ttl: i32,
}

impl Jaccard {
    /// Createa a builder that uses collection of keys
    /// 
    /// ```
    /// use aabel::distances::*;
    /// 
    /// let xs = ["a", "a", "b", "a"];
    /// let ys = ["a", "b", "b", "a", "c"];
    /// let j = Jaccard::from_keys(xs).and_with(ys).compute();
    /// assert_eq!(3, j.common());
    /// assert_eq!(9, j.total());
    /// ```
    pub fn from_keys<I, J>(iter: I) -> JaccardFromKeys<I, J> {
        JaccardFromKeys::new(iter)
    }

    /// Createa a builder that uses collection of key-value pairs
    /// 
    /// ```
    /// use aabel::distances::*;
    /// 
    /// let xs = [("a", 3), ("b", 1)];
    /// let ys = [("a", 2), ("b", 2), ("c", 1)];
    /// let j = Jaccard::from_pairs(xs).and_with(ys).compute();
    /// assert_eq!(3, j.common());
    /// assert_eq!(9, j.total());
    /// ```
    pub fn from_pairs<I, J>(iter: I) -> JaccardFromPairs<I, J> {
        JaccardFromPairs::new(iter)
    }

    /// Returns the number of instances shared between the two collections.
    #[inline]
    pub fn common(&self) -> i32 {
        self.cmn
    }

    /// Returns the the total number of instances.
    #[inline]
    pub fn total(&self) -> i32 {
        self.ttl
    }
}

impl From<Jaccard> for f32 {
    /// Generates the Jacard similarity as a f32 value.
    fn from(j: Jaccard) -> Self {
        j.cmn as f32 / j.ttl as f32
    }
}

/// A trait that defines the functionality of a Jaccard builder,
/// a function which creates a Jaccard similarity value.
pub trait JaccardBulder {
    /// Returns the Jaccard similarity value.
    fn compute(self) -> Jaccard;
}

/// A builder for [`Jaccard`] similarity value that uses
/// collections represented as key-value pairs.
pub struct JaccardFromPairs<I, J> {
    fst: Option<I>,
    snd: Option<J>,
}

impl<I, J> JaccardFromPairs<I, J> {
    fn new(iter: I) -> Self {
        Self {
            fst: Some(iter),
            snd: None,
        }
    }

    /// Adds to the builder the second collection of key-value pairs.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use aabel::distances::*;
    /// 
    /// let xs = [("a", 3), ("b", 1)];
    /// let ys = [("a", 2), ("b", 2), ("c", 1)];
    /// let j: f32 = Jaccard::from_pairs(xs).and_with(ys).compute().into();
    /// assert_eq!(1 as f32 / 3 as f32, j);
    /// ```
    pub fn and_with(mut self, iter: J) -> Self {
        self.snd = Some(iter);
        self
    }
}

impl<K, I: IntoIterator<Item = (K, i32)>, J: IntoIterator<Item = (K, i32)>> JaccardBulder
    for JaccardFromPairs<I, J>
where
    K: Copy + Eq + Hash,
{
    fn compute(self) -> Jaccard {
        let fst = CountedMap::<K, i32>::from_keys_and_values(self.fst.expect(""));
        let snd = CountedMap::<K, i32>::from_keys_and_values(self.snd.expect(""));

        let ttl = fst.total() + snd.total();
        let cmn = *fst.common(&snd).total();

        Jaccard { cmn, ttl }
    }
}

/// A builder for [`Jaccard`] similarity value that uses
/// collections represented as keys.
pub struct JaccardFromKeys<I, J> {
    fst: Option<I>,
    snd: Option<J>,
}

impl<I, J> JaccardFromKeys<I, J> {
    fn new(iter: I) -> Self {
        Self {
            fst: Some(iter),
            snd: None,
        }
    }

    /// Adds to the builder the second collection of keys.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use aabel::distances::*;
    /// 
    /// let xs = ["a", "a", "b", "a"];
    /// let ys = ["a", "b", "b", "a", "c"];
    /// let j: f32 = Jaccard::from_keys(xs).and_with(ys).compute().into();
    /// assert_eq!(1 as f32 / 3 as f32, j);
    /// ```
    pub fn and_with(mut self, iter: J) -> Self {
        self.snd = Some(iter);
        self
    }
}

impl<K, I: IntoIterator<Item = K>, J: IntoIterator<Item = K>> JaccardBulder
    for JaccardFromKeys<I, J>
where
    K: Copy + Eq + Hash,
{
    fn compute(self) -> Jaccard {
        let fst = CountedMap::<K, i32>::from_keys(self.fst.expect(""));
        let snd = CountedMap::<K, i32>::from_keys(self.snd.expect(""));

        let ttl = fst.total() + snd.total();
        let cmn = *fst.common(&snd).total();

        Jaccard { cmn, ttl }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jaccard_from_pairs() {
        let xs = [("a", 3), ("b", 1)];
        let ys = [("a", 2), ("b", 2), ("c", 1)];
        let j = Jaccard::from_pairs(xs).and_with(ys).compute();
        assert_eq!(3, j.common());
        assert_eq!(9, j.total());
    }

    #[test]
    fn jaccard_f32_from_pairs() {
        let xs = [("a", 3), ("b", 1)];
        let ys = [("a", 2), ("b", 2), ("c", 1)];
        let j: f32 = Jaccard::from_pairs(xs).and_with(ys).compute().into();
        assert_eq!(1 as f32 / 3 as f32, j);
    }

    #[test]
    fn jaccard_from_keys() {
        let xs = ["a", "a", "b", "a"];
        let ys = ["a", "b", "b", "a", "c"];
        let j = Jaccard::from_keys(xs).and_with(ys).compute();
        assert_eq!(3, j.common());
        assert_eq!(9, j.total());
    }

    #[test]
    fn jaccard_f32_from_keys() {
        let xs = ["a", "a", "b", "a"];
        let ys = ["a", "b", "b", "a", "c"];
        let j: f32 = Jaccard::from_keys(xs).and_with(ys).compute().into();
        assert_eq!(1 as f32 / 3 as f32, j);
    }
}
