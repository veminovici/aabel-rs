use itertools::Itertools;

/// Returns the [Euclidean](https://en.wikipedia.org/wiki/Euclidean_distance) distance between two collections.
///
/// # Examples
///
/// ```
/// use rust_aabel::distances::euclid;
///
/// let xys = [(3., 0.), (4., 0.)];
/// let it = euclid(xys.into_iter());
/// assert_eq!(5., it)
/// ```
pub fn euclid<I, A, B>(xys: I) -> f32
where
    I: Iterator<Item = (A, B)>,
    A: Into<f32>,
    B: Into<f32>,
{
    fn square_dist<I, J>((x, y): (I, J)) -> f32
    where
        I: Into<f32>,
        J: Into<f32>,
    {
        let x: f32 = x.into();
        let y: f32 = y.into();
        let d = x - y;
        d * d
    }

    xys.map(square_dist)
        .sum1::<f32>()
        .map(|ttl| ttl.sqrt())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclid_() {
        let xys = [(3., 0.), (4., 0.)];
        let it = euclid(xys.into_iter());
        assert_eq!(5., it)
    }
}
