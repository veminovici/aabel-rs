use std::num::NonZeroUsize;

#[inline]
pub fn shingles<'a, T, P>(xs: &'a [T], size: usize, is_start: P) -> Shingles<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    let size = NonZeroUsize::new(size).expect("size is zero");
    Shingles::new(xs, size, is_start)
}

pub struct Shingles<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    v: &'a [T],
    size: NonZeroUsize,
    is_start: P,
}

impl<'a, T: 'a, P> Shingles<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    pub(super) fn new(slice: &'a [T], size: NonZeroUsize, is_start: P) -> Self {
        Self {
            v: slice,
            size,
            is_start,
        }
    }
}

impl<'a, T, P> Iterator for Shingles<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.size.get() > self.v.len() {
            None
        } else {
            if (self.is_start)(&self.v[0]) {
                let ret = Some(&self.v[..self.size.get()]);
                self.v = &self.v[1..];
                ret
            } else {
                self.v = &self.v[1..];
                self.next()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size.get() > self.v.len() {
            (0, Some(0))
        } else {
            let size = self.v.len() - self.size.get() + 1;
            (size, Some(size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shingles_all_() {
        let text = "A spokeperson for the Sudzo Corporation revealed today that studies have shown it is good for people to buy Sudzo products".split_whitespace().collect::<Vec<&str>>();

        let pred = |_: &&str| true;

        let mut ss = shingles(text.as_slice(), 3, pred);
        assert_eq!(Some(["A", "spokeperson", "for"].as_slice()), ss.next());
        assert_eq!(Some(["spokeperson", "for", "the"].as_slice()), ss.next());
        assert_eq!(Some(["for", "the", "Sudzo"].as_slice()), ss.next());
    }

    #[test]
    fn singles_words_() {
        let text = "A spokeperson for the Sudzo Corporation revealed today that studies have shown it is good for people to buy Sudzo products".split_whitespace().collect::<Vec<&str>>();

        let words = ["A", "for", "the", "to", "that"].as_slice();
        let pred = |w: &&str| words.contains(w);

        let mut ss = shingles(text.as_slice(), 3, pred);
        assert_eq!(Some(["A", "spokeperson", "for"].as_slice()), ss.next());
        assert_eq!(Some(["for", "the", "Sudzo"].as_slice()), ss.next());
        assert_eq!(Some(["the", "Sudzo", "Corporation"].as_slice()), ss.next());
    }
}
