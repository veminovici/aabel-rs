use itertools::Itertools;

/// Returns the [Manhattan](https://en.wikipedia.org/wiki/Taxicab_geometry) distance between two collections.
///
/// # Examples
///
/// ```
/// use rust_aabel::distances::manhattan;
///
/// let xys = [(3., 0.), (4., 0.)];
/// let it = manhattan(xys.into_iter());
/// assert_eq!(7., it)
/// ```
pub fn manhattan<I, A, B>(xys: I) -> f32
where
    I: Iterator<Item = (A, B)>,
    A: Into<f32>,
    B: Into<f32>,
{
    fn dist<I, J>((x, y): (I, J)) -> f32
    where
        I: Into<f32>,
        J: Into<f32>,
    {
        let x: f32 = x.into();
        let y: f32 = y.into();
        let d = x - y;
        d.abs()
    }

    xys.map(dist).sum1::<f32>().unwrap()
}
