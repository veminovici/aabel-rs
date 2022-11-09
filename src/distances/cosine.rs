pub fn cosine<I, A, B>(xys: I) -> f32
where
    I: Iterator<Item = (A, B)>,
    A: Into<f32> + Copy,
    B: Into<f32> + Copy,
{
    fn product<I, J>(x: &I, y: &J) -> f32
    where
        I: Into<f32> + Copy,
        J: Into<f32> + Copy,
    {
        let x: f32 = (*x).into();
        let y: f32 = (*y).into();
        x * y
    }

    fn square<I>(x: I) -> f32
    where
        I: Into<f32>,
    {
        let x: f32 = x.into();
        x * x
    }

    let (prod, xsquare, ysquare) =
        xys.fold((0_f32, 0_f32, 0_f32), |(prod, xsquare, ysquare), (x, y)| {
            let prod = prod + product(&x, &y);
            let xsquare = xsquare + square(x);
            let ysquare = ysquare + square(y);

            (prod, xsquare, ysquare)
        });

    let denom = xsquare.sqrt() * ysquare.sqrt();
    if denom == 0. {
        0.
    } else {
        prod / denom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_() {
        let xys = [(1., 0.), (1., 0.)];
        let it = cosine(xys.into_iter());
        assert_eq!(0., it);

        let xys = [(1., 2.), (2., 1.), (-1., 1.)];
        let it = cosine(xys.into_iter());
        assert!((it - 0.5).abs() <= 0.01);
    }
}
