/// Returns the [Hamming](https://en.wikipedia.org/wiki/Hamming_distance) distance between two collections.
///
/// # Examples
///
/// ```
/// use rust_aabel::distances::hamming;
///
/// let xys = [('k', 'k'), ('a', 'a'), ('r', 't'), ('o', 'h'), ('l', 'r'), ('i', 'i'), ('n', 'n')];
/// let it = hamming(xys.into_iter());
/// assert_eq!(3, it)
/// ```
pub fn hamming<I, A>(xys: I) -> usize
where
    I: Iterator<Item = (A, A)>,
    A: Eq,
{
    xys.filter_map(|(x, y)| if x == y { None } else { Some(1) })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_() {
        let xys = [
            ('k', 'k'),
            ('a', 'a'),
            ('r', 't'),
            ('o', 'h'),
            ('l', 'r'),
            ('i', 'i'),
            ('n', 'n'),
        ];
        let it = hamming(xys.into_iter());
        assert_eq!(3, it)
    }
}
