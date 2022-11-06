use std::hash::Hash;

use itertools::Itertools;

use crate::counted_bag::CountedBag;

/// Retrieves a distance.
pub trait Distance: Iterator {
    /// Returns the [Euclidean](https://en.wikipedia.org/wiki/Euclidean_distance) distance between two collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::distances::Distance;
    ///
    /// let it = [3., 4.].into_iter().euclid([0., 0.]);
    /// assert_eq!(5., it)
    /// ```
    fn euclid<J>(self, ys: J) -> f32
    where
        J: IntoIterator<Item = Self::Item>,
        Self::Item: Into<f32>,
        Self: Sized,
    {
        let xys = self.into_iter().zip_eq(ys);
        super::euclid(xys)
    }

    /// Returns the [Manhattan](https://en.wikipedia.org/wiki/Taxicab_geometry) distance between two collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::distances::Distance;
    ///
    /// let it = [3., 4.].into_iter().manhattan([0., 0.]);
    /// assert_eq!(7., it)
    /// ```
    fn manhattan<J>(self, ys: J) -> f32
    where
        J: IntoIterator<Item = Self::Item>,
        Self::Item: Into<f32>,
        Self: Sized,
    {
        let xys = self.into_iter().zip_eq(ys);
        super::manhattan(xys)
    }

    /// Returns the [Hamming](https://en.wikipedia.org/wiki/Hamming_distance) distance between two collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::distances::Distance;
    ///
    /// let it = ['k', 'a', 'r', 'o', 'l', 'i', 'n'].into_iter().hamming(['k', 'a', 't', 'h', 'r', 'i', 'n']);
    /// assert_eq!(3, it)
    /// ```
    fn hamming<J>(self, ys: J) -> usize
    where
        J: IntoIterator<Item = Self::Item>,
        Self::Item: Eq,
        Self: Sized,
    {
        let xys = self.into_iter().zip_eq(ys);
        super::hamming(xys)
    }

    /// Returns the Jaccard distance between two counted collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::distances::Distance;
    /// let xs = [('a', 1), ('b', 2), ('c', 3)];
    /// let ys = [('b', 1), ('c', 2), ('d', 3)];
    /// let it = xs.into_iter().jaccard(ys);
    /// assert_eq!(it, 0.25);
    /// ```
    fn jaccard<K, J>(self, ys: J) -> f32
    where
        J: IntoIterator<Item = Self::Item>,
        Self: Iterator<Item = (K, u32)>,
        Self: Sized,
        K: Eq + Hash,
    {
        let xs = CountedBag::<K>::from_iter(self);
        let ys = CountedBag::<K>::from_iter(ys);
        let j = super::jaccard(&xs, &ys);
        j.value()
    }

    /// Returns the Jaccard distance between two counted collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::distances::Distance;
    /// let xs = ['a','b', 'b', 'c', 'c', 'c'];
    /// let ys = ['b', 'c', 'c', 'd', 'd', 'd'];
    /// let it = xs.into_iter().jaccard1(ys);
    /// assert_eq!(it, 0.25);
    /// ```
    fn jaccard1<J>(self, ys: J) -> f32
    where
        J: IntoIterator<Item = Self::Item>,
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let xs = CountedBag::<Self::Item>::from_keys(self);
        let ys = CountedBag::<Self::Item>::from_keys(ys.into_iter());
        let j = super::jaccard(&xs, &ys);
        j.value()
    }
}

impl<T: ?Sized> Distance for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::Distance;

    #[test]
    fn euclid_() {
        let it = [3., 4.].into_iter().euclid([0., 0.]);
        assert_eq!(5., it)
    }

    #[test]
    fn manhattan_() {
        let it = [3., 4.].into_iter().manhattan([0., 0.]);
        assert_eq!(7., it)
    }

    #[test]
    fn jaccard_() {
        let xs = [('a', 1), ('b', 2), ('c', 3)];
        let ys = [('b', 1), ('c', 2), ('d', 3)];
        let it = xs.into_iter().jaccard(ys);
        assert_eq!(it, 0.25);
    }

    #[test]
    fn jaccard_1_() {
        let xs = ['a', 'b', 'b', 'c', 'c', 'c'];
        let ys = ['b', 'c', 'c', 'd', 'd', 'd'];
        let it = xs.into_iter().jaccard1(ys);
        assert_eq!(it, 0.25);
    }

    #[test]
    fn hamming_() {
        let it = ['k', 'a', 'r', 'o', 'l', 'i', 'n']
            .into_iter()
            .hamming(['k', 'a', 't', 'h', 'r', 'i', 'n']);
        assert_eq!(3, it);

        let it = "karolin"
            .as_bytes()
            .into_iter()
            .hamming("kathrin".as_bytes());
        assert_eq!(3, it);
    }
}
