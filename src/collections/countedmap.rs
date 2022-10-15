use std::borrow::Borrow;
use std::collections::{
    hash_map::{Entry, Iter, RandomState},
    HashMap,
};
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::ops::AddAssign;

use crate::{FromKeys, FromKeysAndValues};

pub struct CountedMap<K, V, S = RandomState> {
    base: HashMap<K, V, S>,
    ttl: V,
}

impl<K, V, S> Default for CountedMap<K, V, S>
where
    S: Default,
    V: num::Zero,
{
    #[inline]
    fn default() -> Self {
        Self {
            base: Default::default(),
            ttl: V::zero(),
        }
    }
}

//
// Formatting
//

impl<K, V, S> Debug for CountedMap<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CountedMap")
            .field("total", &self.ttl)
            .field("base", &self.base)
            .finish()
    }
}

//
// Functionalities
//

impl<K, V, S> CountedMap<K, V, S> {
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.base.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    #[inline]
    pub fn total(&self) -> &V {
        &self.ttl
    }
}

impl<K, V, S> CountedMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    pub fn entry(&mut self, k: K) -> Entry<'_, K, V> {
        self.base.entry(k)
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get(k)
    }
}

impl<K, V, S> CountedMap<K, V, S>
where
    K: Eq + Hash,
    V: AddAssign + num::One + num::Zero,
    S: Default + BuildHasher,
{
    #[inline]
    pub fn insert(&mut self, k: K) -> Option<&V> {
        self.ttl += V::one();

        let x = self
            .entry(k)
            .and_modify(|e| *e += V::one())
            .or_insert(V::one());

        Some(x)
    }
}

#[inline]
fn min_value<V: Copy + Ord>(a: &V, b: &V) -> V {
    let a = *a;
    let b = *b;

    if a <= b {
        a
    } else {
        b
    }
}

impl<K, V, S> CountedMap<K, V, S>
where
    S: Default + BuildHasher,
    K: Copy + Eq + Hash,
    V: AddAssign + Copy + Ord + num::Zero,
{
    #[inline]
    fn insert_value(&mut self, k: K, v: V) -> Option<&V> {
        self.ttl += v;

        let x = self.entry(k).or_insert(v);
        Some(x)
    }

    #[inline]
    fn get_cmn(&self, k: &K, other: &V) -> Option<V> {
        self.get(k).map(|v| min_value(v, other))
    }

    #[inline]
    pub fn common(&self, other: &CountedMap<K, V, S>) -> Self {
        let mut cmn = Self::default();

        other.iter().fold(&mut cmn, |acc, (k, other)| {
            let _ = self.get_cmn(k, other).and_then(|v| acc.insert_value(*k, v));
            acc
        });

        cmn
    }
}

//
// Constructors
//

impl<K, V, S> FromKeys<K> for CountedMap<K, V, S>
where
    K: Eq + Hash,
    V: AddAssign + Default + num::One + num::Zero,
    S: Default + BuildHasher,
{
    fn from_keys<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut bag = Self::default();
        for k in iter {
            bag.insert(k);
        }
        bag
    }
}

impl<K, V, S> FromKeysAndValues<K, V> for CountedMap<K, V, S>
where
    K: Copy + Eq + Hash,
    V: AddAssign + Copy + Ord + num::Zero,
    S: Default + BuildHasher,
{
    fn from_keys_and_values<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut bag = Self::default();
        for (k, v) in iter {
            bag.insert_value(k, v);
        }
        bag
    }
}

#[cfg(test)]
mod tests {
    use super::CountedMap;
    use crate::{FromKeys, FromKeysAndValues};

    #[test]
    fn from_keys_() {
        let keys = ["a", "b", "c", "a"];
        let cmap = CountedMap::<&str, i32>::from_keys(keys);

        assert_eq!(cmap.len(), 3);
        assert!(!cmap.is_empty());
        assert_eq!(cmap.get(&"a"), Some(&2));
        assert_eq!(cmap.get(&"b"), Some(&1));
        assert_eq!(cmap.get(&"c"), Some(&1));
        assert_eq!(cmap.total(), &4);

        eprintln!("CMAP: {:?}", &cmap);
    }

    #[test]
    fn from_kesy_and_values_() {
        let kvs = [("a", 2), ("b", 1), ("c", 1)];
        let cmap = CountedMap::<&str, i32>::from_keys_and_values(kvs);

        assert_eq!(cmap.len(), 3);
        assert_eq!(cmap.get(&"a"), Some(&2));
        assert_eq!(cmap.get(&"b"), Some(&1));
        assert_eq!(cmap.get(&"c"), Some(&1));
        assert_eq!(cmap.total(), &4);
    }

    #[test]
    fn common_() {
        let xs = CountedMap::<&str, i32>::from_keys_and_values([("a", 3), ("b", 1)]);
        let ys = CountedMap::<&str, i32>::from_keys_and_values([("a", 2), ("b", 2), ("c", 1)]);
        let cmn = xs.common(&ys);

        assert_eq!(cmn.len(), 2);
        assert_eq!(cmn.get(&"a"), Some(&2));
        assert_eq!(cmn.get(&"b"), Some(&1));
        assert_eq!(cmn.get(&"c"), None);
        assert_eq!(cmn.total(), &3);
    }
}
