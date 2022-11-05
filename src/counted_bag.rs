//! A store based on the [`HashMap`] where we can store counted bags.
//!

use std::{
    borrow::Borrow,
    collections::{
        hash_map::{IntoIter as HMIntoIter, Iter as HMIter, Keys, RandomState},
        HashMap,
    },
    fmt::Debug,
    hash::{BuildHasher, Hash},
};

/// Stores the total number of occurences for each elements as well
/// as the total number of elements.
///
/// # Examples
///
/// ```
/// use rust_aabel::counted_bag::CountedBag;
/// let mut cs= CountedBag::<char>::new();
/// cs.insert('a');
/// cs.insert('b');
///
/// for key in cs.keys() {
///    println!("{key}");
/// }
/// ```
pub struct CountedBag<K, S = RandomState> {
    hmap: HashMap<K, u32, S>,
    total: u32,
}

impl<K, S> Default for CountedBag<K, S>
where
    S: Default,
{
    /// Creates an empty `CountedBag`.
    ///
    /// The counted set is initially created with a capacity of 0, so it will not allocate until
    /// it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    /// let mut cs= CountedBag::<char>::default();
    /// ```
    fn default() -> Self {
        Self {
            hmap: Default::default(),
            total: 0,
        }
    }
}

impl<K, S> CountedBag<K, S>
where
    S: Default,
{
    /// Creates an empty `CountedBag`.
    ///
    /// The counted set is initially created with a capacity of 0, so it will not allocate until
    /// it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    /// let mut cs= CountedBag::<char>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K, S> CountedBag<K, S> {
    /// Returns the number of distinct elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    /// let mut cs= CountedBag::<char>::default();
    ///
    /// let x = cs.insert('a');
    /// assert_eq!(x, 1);
    ///
    /// let x = cs.insert('b');
    /// assert_eq!(x, 1);
    ///
    ///assert_eq!(2, cs.len());
    /// ```
    pub fn len(&self) -> usize {
        self.hmap.len()
    }

    /// Returns true if the set contains no elements.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    /// let mut cs= CountedBag::<char>::default();
    ///
    /// assert!(cs.is_empty());
    /// let _x = cs.insert('a');
    /// assert!(!cs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.hmap.is_empty()
    }

    /// An iterator visiting all distinct items in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    /// let mut cs= CountedBag::<char>::new();
    /// cs.insert('a');
    /// cs.insert('b');
    ///
    /// for key in cs.keys() {
    ///    println!("{key}");
    /// }
    /// ```
    pub fn keys(&self) -> Keys<'_, K, u32> {
        self.hmap.keys()
    }

    /// Returns the total number of elements.
    pub fn total(&self) -> u32 {
        self.total
    }
}

impl<K, S> CountedBag<K, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Returns a reference to the number of occurences for the corresponding key.
    ///
    /// The key may be any borrowed form of the map's key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    ///
    /// let mut cs = CountedBag::<char>::new();
    /// cs.insert('a');
    /// assert_eq!(cs.get(&'a'), Some(&1));
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&u32>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hmap.get(k)
    }

    /// Inserts a new occurence of the key.
    /// The function returns the number of occurences of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    ///
    /// let mut cs= CountedBag::<char>::new();
    /// let x = cs.insert('a');
    /// assert_eq!(x, 1);
    ///
    /// let x = cs.insert('b');
    /// assert_eq!(x, 1);
    ///
    /// let x = cs.insert('a');
    /// assert_eq!(x, 2);
    /// ```
    pub fn insert(&mut self, k: K) -> u32 {
        self.total += 1;

        let count = self.get(&k).map_or(1, |i| *i + 1);
        self.hmap.insert(k, count).map_or(1, |x| x + 1)
    }

    /// create a counted bag from a collection of keys.
    pub fn from_keys<J>(xs: J) -> Self
    where
        J: Iterator<Item = K>,
        S: Default,
    {
        let mut cs = Self::default();

        for k in xs {
            let _ = cs.insert(k);
        }

        cs
    }
}

impl<K, S> CountedBag<K, S> {
    /// An iterator visiting all distinct items and their count in an arbitrary order.
    /// The iterator element type is (&'a K, &'a V)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    ///
    /// let mut cs = CountedBag::<char>::new();
    /// cs.insert('a');
    /// cs.insert('b');
    /// cs.insert('a');
    ///
    /// for (key, val) in cs.iter() {
    ///     println!("key: {key}, val: {val}");
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K> {
        Iter {
            base: self.hmap.iter(),
        }
    }
}

/// An iterator over the entries of a `CountedBag`.
///
/// The `struct` is created by the [`iter`] method on [`CountedBag`]. See its documentation for more.
///
/// [`iter`]: CountedBag::iter
///
/// # Example
///
/// ```
/// use rust_aabel::counted_bag::CountedBag;
///
/// let mut cs = CountedBag::<char>::new();
/// cs.insert('a');
/// cs.insert('b');
/// cs.insert('a');
/// let iter = cs.iter();
/// ```
pub struct Iter<'a, K: 'a> {
    base: HMIter<'a, K, u32>,
}

impl<'a, K> Clone for Iter<'a, K> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            base: self.base.clone(),
        }
    }
}

impl<'a, K> Debug for Iter<'a, K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = (&'a K, &'a u32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

/// An owning iterator over the entries of a `CountedBag`.
///
/// This `struct` is created by the [`into_iter`] method on [`CountedBag`] (provided by the [`IntoIterator`] trait).
/// See its documentation for more details.
///
/// [`into_iter`]: IntoIterator::into_iter
/// [`IntoIterator`]: crate::iter::IntoIterator
pub struct IntoIter<K> {
    base: HMIntoIter<K, u32>,
}

impl<'a, K, S> IntoIterator for &'a CountedBag<K, S> {
    type Item = (&'a K, &'a u32);
    type IntoIter = Iter<'a, K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, S> IntoIterator for CountedBag<K, S> {
    type Item = (K, u32);
    type IntoIter = IntoIter<K>;

    /// Creates a consuming iterator, that is, one that moves each element out of the
    /// set in arbitrary order. The set cannot be used after calling this.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_aabel::counted_bag::CountedBag;
    ///
    /// let mut cs = CountedBag::<char>::new();
    /// cs.insert('a');
    /// cs.insert('b');
    /// cs.insert('a');
    ///
    /// // Not possible with .iter()
    /// let _vec: Vec<(char, u32)> = cs.into_iter().collect();
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            base: self.hmap.into_iter(),
        }
    }
}

impl<K> Iterator for IntoIter<K> {
    type Item = (K, u32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl<K, S> FromIterator<(K, u32)> for CountedBag<K, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, u32)>>(iter: T) -> Self {
        let hmap = HashMap::from_iter(iter);
        let total = hmap.values().sum();
        CountedBag { hmap, total }
    }
}

impl<K, const N: usize> From<[(K, u32); N]> for CountedBag<K, RandomState>
where
    K: Eq + Hash,
{
    fn from(arr: [(K, u32); N]) -> Self {
        Self::from_iter(arr)
    }
}

//
// Intersection
//

/// A lazy iterator producing elements in the intersection of [`CountedBag`]s.
///
/// The `struct` is created by the [`intersection`] method on [`CountedBag`]. See the documentation for more.
///
/// [`intersection`]: CountedBag::intersection
///
/// # Examples
///
/// ```
/// use rust_aabel::counted_bag::CountedBag;
///
/// let mut xs = CountedBag::<char>::new();
/// xs.insert('a');
/// xs.insert('b');
/// xs.insert('a');
/// xs.insert('x');
/// let mut ys = CountedBag::<char>::new();
/// ys.insert('a');
/// ys.insert('b');
/// ys.insert('c');
/// let intersection = xs.intersection(&ys);
/// ```
pub struct Intersection<'a, K: 'a, S: 'a> {
    // iterator of the first set
    iter: Iter<'a, K>,
    // the second set
    other: &'a CountedBag<K, S>,
}

impl<K, S> Clone for Intersection<'_, K, S> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a, K, S> Iterator for Intersection<'a, K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, u32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (key, val) = self.iter.next()?;
            if let Some(val1) = self.other.get(key) {
                if *val <= *val1 {
                    return Some((key, *val));
                } else {
                    return Some((key, *val1));
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

impl<K, S> CountedBag<K, S> {
    pub fn intersection<'a>(&'a self, other: &'a CountedBag<K, S>) -> Intersection<'a, K, S> {
        if self.len() <= other.len() {
            Intersection {
                iter: self.iter(),
                other,
            }
        } else {
            Intersection {
                iter: other.iter(),
                other: self,
            }
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_() {
        let mut cs = CountedBag::<char>::new();
        let x = cs.insert('a');
        assert_eq!(x, 1);

        let x = cs.insert('b');
        assert_eq!(x, 1);

        assert_eq!(2, cs.len());
    }

    #[test]
    fn is_empty_() {
        let mut cs = CountedBag::<char>::new();
        assert!(cs.is_empty());

        let _x = cs.insert('a');
        assert!(!cs.is_empty());
    }

    #[test]
    fn keys_() {
        let mut cs = CountedBag::<char>::new();
        cs.insert('a');
        cs.insert('b');

        let mut c = 0;
        for key in cs.keys() {
            println!("{key}");
            c += 1;
        }

        assert_eq!(c, 2);
    }

    #[test]
    fn total_() {
        let mut cs = CountedBag::<char>::new();
        cs.insert('a');
        cs.insert('b');
        cs.insert('a');
        assert_eq!(cs.total(), 3);
    }

    #[test]
    fn get_() {
        let mut cs = CountedBag::<char>::new();
        let x = cs.insert('a');
        assert_eq!(x, 1);

        let x = cs.insert('b');
        assert_eq!(x, 1);

        let x = cs.get(&'a');
        assert_eq!(x, Some(&1));
    }

    #[test]
    fn insert_() {
        let mut cs = CountedBag::<char>::new();
        let x = cs.insert('a');
        assert_eq!(x, 1);

        let x = cs.insert('b');
        assert_eq!(x, 1);

        let x = cs.insert('a');
        assert_eq!(x, 2);
    }

    #[test]
    fn from_iter_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);
        assert_eq!(cs.get(&'a'), Some(&2));
        assert_eq!(cs.get(&'b'), Some(&1));
        assert_eq!(cs.total(), 3);
    }

    #[test]
    fn iter_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);

        let iter = cs.iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn clone_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);

        let iter = cs.iter().clone();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn debug_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);
        let s = format!("{:#?}", cs.iter());
        assert!(!s.is_empty())
    }

    #[test]
    fn into_iter_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);

        let vec: Vec<(char, u32)> = cs.into_iter().collect();
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn into_iter_1_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from_iter(xs);

        let vec: Vec<(&char, &u32)> = (&cs).into_iter().collect();
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn from_arr_() {
        let xs = [('a', 2), ('b', 1)];
        let cs = CountedBag::<char>::from(xs);

        let vec: Vec<(char, u32)> = cs.into_iter().collect();
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn from_keys() {
        let cs = CountedBag::<char>::from_keys(['a', 'b', 'a', 'a', 'c', 'b'].into_iter());
        let v = cs.get(&'a');
        assert_eq!(v, Some(&3));
    }

    #[test]
    fn intersection_() {
        let xs = [('a', 2), ('b', 1), ('x', 10)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('a', 1), ('b', 1), ('c', 20)];
        let ys = CountedBag::<char>::from_iter(ys);

        let intersection = xs.intersection(&ys);
        let iter = intersection.into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn intersection_1_() {
        let xs = [('a', 2), ('b', 1), ('x', 10)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('a', 1), ('b', 1), ('c', 20), ('d', 30)];
        let ys = CountedBag::<char>::from_iter(ys);

        let intersection = xs.intersection(&ys);
        let iter = intersection.into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn intersection_2_() {
        let xs = [('a', 1), ('b', 1), ('c', 20), ('d', 30)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('a', 2), ('b', 1), ('x', 10)];
        let ys = CountedBag::<char>::from_iter(ys);

        let intersection = xs.intersection(&ys);
        let iter = intersection.into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn intersection_clone_() {
        let xs = [('a', 2), ('b', 1), ('x', 10)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('a', 1), ('b', 1), ('c', 20)];
        let ys = CountedBag::<char>::from_iter(ys);

        let intersection = xs.intersection(&ys).clone();
        let iter = intersection.into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn intersection_counted_bag() {
        let xs = [('a', 2), ('b', 1), ('x', 10)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('a', 1), ('b', 1), ('c', 20)];
        let ys = CountedBag::<char>::from_iter(ys);

        let intersection = xs.intersection(&ys);
        let intersection = CountedBag::<&char>::from_iter(intersection);
        assert_eq!(intersection.total(), 2);
    }
}
