use std::ptr;

/// Builder that can generate the permutations
/// for a slice set of a given size.
///
/// # Examples
///
/// ```
/// use aabel_rs::collections::Permutations;
///
/// let source = &mut [1, 2, 3, 4];
/// let mut permutations = Permutations::new(4, source);
/// let results = permutations.generate();
/// assert_eq!(24, results.len());
/// ```
pub struct Permutations<'a, T> {
    /// The length of the slice.
    len: usize,
    /// The slice of data
    arr: &'a mut [T],
}

impl<'a, T> Permutations<'a, T> {
    /// Creates a new permutation builder instance.
    pub fn new(len: usize, arr: &'a mut [T]) -> Self {
        Self { arr, len }
    }

    fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            let pa: *mut T = &mut self.arr[a];
            let pb: *mut T = &mut self.arr[b];
            ptr::swap(pa, pb);
        }
    }
}

impl<'a, T> Permutations<'a, T>
where
    T: Clone + Default,
{
    fn to_result(&self) -> Vec<T>
    where
        T: Clone + Default,
    {
        let mut res: Vec<T> = vec![Default::default(); self.len];
        res.clone_from_slice(self.arr);
        res
    }

    /// Gnerates all permutations.
    pub fn generate(&mut self) -> Vec<Vec<T>> {
        let mut results: Vec<Vec<T>> = vec![];
        results.push(self.to_result());

        let mut stack = vec![0; self.len];
        let mut i = 1;

        while i < self.len {
            if stack[i] < i {
                let (a, b) = if i % 2 == 0 { (0, i) } else { (stack[i], i) };

                self.swap(a, b);

                results.push(self.to_result());

                stack[i] += 1;
                i = 1;
            } else {
                stack[i] = 0;
                i += 1;
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn permutations_() {
        let xs = &mut [1, 2, 3, 4];
        let mut permutations = Permutations::new(4, xs);

        let results = permutations.generate();
        assert_eq!(24, results.len());

        let mut rng = thread_rng();
        let res = results.choose(&mut rng).unwrap();
        println!("ARR: {res:?}");
    }
}
