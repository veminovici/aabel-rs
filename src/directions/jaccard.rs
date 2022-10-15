use crate::{CountedMap, FromKeys, FromKeysAndValues};
use std::hash::Hash;

pub struct Jaccard {
    cmn: i32,
    ttl: i32,
}

impl Jaccard {
    pub fn from_keys<I, J>(iter: I) -> JaccardFromKeys<I, J> {
        JaccardFromKeys::new(iter)
    }

    pub fn from_pairs<I, J>(iter: I) -> JaccardFromPairs<I, J> {
        JaccardFromPairs::new(iter)
    }
}

impl From<Jaccard> for f32 {
    fn from(j: Jaccard) -> Self {
        j.cmn as f32 / j.ttl as f32
    }
}

//
// From pairs of keys and values
//

pub trait JaccardBulder {
    fn compute(self) -> Jaccard;
}

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

//
// From keys
//

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
        assert_eq!(3, j.cmn);
        assert_eq!(9, j.ttl);
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
        let jsim = Jaccard::from_keys(xs).and_with(ys).compute();
        assert_eq!(3, jsim.cmn);
        assert_eq!(9, jsim.ttl);
    }

    #[test]
    fn jaccard_f32_from_keys() {
        let xs = ["a", "a", "b", "a"];
        let ys = ["a", "b", "b", "a", "c"];
        let jsim: f32 = Jaccard::from_keys(xs).and_with(ys).compute().into();
        assert_eq!(1 as f32 / 3 as f32, jsim);
    }
}
