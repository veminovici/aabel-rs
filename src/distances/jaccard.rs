use crate::counted_bag::CountedBag;
use std::hash::{BuildHasher, Hash};

/// Represents the Jaccard similarity value.
pub struct JaccardSim {
    pub(crate) numer: u32,
    pub(crate) denom: u32,
}

impl JaccardSim {
    pub fn value(&self) -> f32 {
        self.numer as f32 / self.denom as f32
    }
}

/// Returns the [Jaccard](https://en.wikipedia.org/wiki/Jaccard_index) index between two counted bags.
///
/// # Examples
///
/// ```
/// use rust_aabel::counted_bag::CountedBag;
/// use rust_aabel::distances::jaccard;
///
/// let xs = [('a', 1), ('b', 2), ('c', 3)];
/// let xs = CountedBag::<char>::from_iter(xs);
///
/// let ys = [('b', 1), ('c', 2), ('d', 3)];
/// let ys = CountedBag::<char>::from_iter(ys);
///
/// let j = jaccard(&xs, &ys);
/// assert_eq!(j.value(), 0.25);
/// ```
pub fn jaccard<'a, K, S>(first: &CountedBag<K, S>, second: &CountedBag<K, S>) -> JaccardSim
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    let union = first.total() + second.total();
    let intersection = CountedBag::<_, S>::from_iter(first.intersection(second)).total();
    JaccardSim {
        numer: intersection,
        denom: union,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jaccard_ratio_() {
        let xs = [('a', 1), ('b', 2), ('c', 3)];
        let xs = CountedBag::<char>::from_iter(xs);

        let ys = [('b', 1), ('c', 2), ('d', 3)];
        let ys = CountedBag::<char>::from_iter(ys);

        let j = jaccard(&xs, &ys);
        assert_eq!(j.numer, 3);
        assert_eq!(j.denom, 12);
        assert_eq!(j.value(), 0.25);
    }

    #[test]
    fn jaccard_() {
        let xs = [("a", 3), ("b", 1)];
        let xs = CountedBag::<&str>::from_iter(xs);

        let ys = [("a", 2), ("b", 2), ("c", 1)];
        let ys = CountedBag::<&str>::from_iter(ys);

        let j = jaccard(&xs, &ys);
        assert_eq!(j.numer, 3);
        assert_eq!(j.denom, 9);
        assert_eq!(j.value(), 1. / 3.);
    }
}
