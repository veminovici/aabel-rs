use super::Bit;
use std::fmt::{Binary, Debug, Display, LowerHex, UpperHex};

const MASKS_SET: [u8; 8] = [1 << 7, 1 << 6, 1 << 5, 1 << 4, 1 << 3, 1 << 2, 1 << 1, 1];
const MASKS_RESET: [u8; 8] = [
    !(1 << 7),
    !(1 << 6),
    !(1 << 5),
    !(1 << 4),
    !(1 << 3),
    !(1 << 2),
    !(1 << 1),
    !1,
];

// #[cfg(test)]
// #[macro_use(quickcheck)]

/// Representation of a byte
///
/// # Examples
///
/// ```
/// use aabel_rs::bits::{Bit, Byte};
///
/// let byte = Byte::from(10);
///
/// let bit = byte.get_bit(4);
/// assert_eq!(bit, 1.into());
///
/// let mut iter = byte.iter();
/// assert_eq!(iter.next(), Some(Bit::Zero));
/// ```
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Byte(u8);

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{:08b})", self.0, self.0)
    }
}

impl LowerHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

impl UpperHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.0)
    }
}

impl Binary for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl From<u8> for Byte {
    #[inline]
    fn from(value: u8) -> Self {
        Byte(value)
    }
}

impl From<Byte> for u8 {
    #[inline]
    fn from(byte: Byte) -> Self {
        byte.0
    }
}

impl FromIterator<Bit> for Byte {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        iter.into_iter()
            .enumerate()
            .fold(0.into(), |acc, (bit, item)| {
                if item == Bit::One {
                    acc.set_bit(bit as u8)
                } else {
                    acc
                }
            })
    }
}

impl FromIterator<bool> for Byte {
    #[inline]
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Byte::from_iter(iter.into_iter().map(|b| Bit::from(b)))
    }
}

impl FromIterator<u8> for Byte {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Byte::from_iter(iter.into_iter().map(|x| Bit::from(x)))
    }
}

impl Byte {
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        self.0 == 1
    }

    #[inline]
    pub fn get_bit(&self, bit: u8) -> Bit {
        let mask = MASKS_SET[bit as usize];
        (self.0 & mask).into()
    }

    #[inline]
    pub fn set_bit(self, bit: u8) -> Self {
        let mask = MASKS_SET[bit as usize];
        Self(self.0 | mask)
    }

    #[inline]
    pub fn reset_bit(self, bit: u8) -> Self {
        let mask = MASKS_RESET[bit as usize];
        Self(self.0 & mask)
    }

    #[inline]
    pub fn toggle_bit(self, bit: u8) -> Self {
        let mask = MASKS_SET[bit as usize];
        Self(self.0 ^ mask)
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            byte: self.clone(),
            crnt: 0,
        }
    }
}

impl IntoIterator for Byte {
    type Item = Bit;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            byte: self,
            crnt: 0,
        }
    }
}

/// Represents an iterator over a byte.
/// The elements of the iteration are [`Bit`] instances.
pub struct Iter {
    byte: Byte,
    crnt: u8,
}

impl Iterator for Iter {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.crnt > 7 {
            None
        } else {
            let res = self.byte.get_bit(self.crnt);
            self.crnt += 1;
            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    impl quickcheck::Arbitrary for Byte {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            u8::arbitrary(g).into()
        }
    }

    /// Argument for building the Byte from a list of u8 elements.
    #[derive(Clone, Copy, Debug)]
    pub struct Elements01 {
        pub xs: [u8; 8],
    }

    impl quickcheck::Arbitrary for Elements01 {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let u0 = u8::arbitrary(g) % 2;
            let u1 = u8::arbitrary(g) % 2;
            let u2 = u8::arbitrary(g) % 2;
            let u3 = u8::arbitrary(g) % 2;
            let u4 = u8::arbitrary(g) % 2;
            let u5 = u8::arbitrary(g) % 2;
            let u6 = u8::arbitrary(g) % 2;
            let u7 = u8::arbitrary(g) % 2;

            let xs = [u0, u1, u2, u3, u4, u5, u6, u7];
            Elements01 { xs }
        }
    }

    /// Arguments for building the Byte for a list of bool elements.
    #[derive(Clone, Copy, Debug)]
    pub struct ElementsBool {
        pub xs: [bool; 8],
    }

    impl quickcheck::Arbitrary for ElementsBool {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let u0 = u8::arbitrary(g) % 2;
            let u1 = u8::arbitrary(g) % 2;
            let u2 = u8::arbitrary(g) % 2;
            let u3 = u8::arbitrary(g) % 2;
            let u4 = u8::arbitrary(g) % 2;
            let u5 = u8::arbitrary(g) % 2;
            let u6 = u8::arbitrary(g) % 2;
            let u7 = u8::arbitrary(g) % 2;

            let xs = [u0, u1, u2, u3, u4, u5, u6, u7].map(|u| if u == 0 { false } else { true });
            ElementsBool { xs }
        }
    }

    #[quickcheck]
    fn prop_from_into_(x: u8) -> bool {
        let byte = Byte::from(x);
        x == byte.into()
    }

    #[quickcheck]
    fn prop_display_(byte: Byte) -> bool {
        !format!("byte: {byte}").is_empty()
    }

    #[quickcheck]
    fn prop_set_get_(byte: Byte, bit: u8) -> bool {
        let bit = bit % 8;
        let byte = byte.set_bit(bit);
        Bit::One == byte.get_bit(bit)
    }

    #[quickcheck]
    fn prop_reset_get_(byte: Byte, bit: u8) -> bool {
        let bit = bit % 8;
        let byte = byte.reset_bit(bit);
        Bit::Zero == byte.get_bit(bit)
    }

    #[quickcheck]
    fn prop_toggle_(byte: Byte, bit: u8) -> bool {
        let bit = bit % 8;

        let orig = byte.get_bit(bit);
        let byte = byte.toggle_bit(bit);
        let upd = byte.get_bit(bit);

        orig != upd
    }

    #[quickcheck]
    fn prop_eq_(byte: Byte) -> bool {
        let byte1 = byte.clone();
        byte1 == byte
    }

    #[quickcheck]
    fn prop_diff_(byte: Byte, bit: u8) -> bool {
        let bit = bit % 8;
        let byte1 = byte.toggle_bit(bit);
        byte1 != byte
    }

    #[quickcheck]
    fn prop_ord_(byte: Byte, bit: u8) -> bool {
        let bit = bit % 8;
        let byte1 = byte.set_bit(bit);
        byte <= byte1
    }

    #[quickcheck]
    fn prop_from_u8_(elements: Elements01) -> bool {
        let byte = Byte::from_iter(elements.xs);
        let iter = byte.iter().map(|b| u8::from(b));
        iter.zip(elements.xs).all(|(i, x)| i == x)
    }

    #[quickcheck]
    fn prop_from_bool_(elements: ElementsBool) -> bool {
        let byte = Byte::from_iter(elements.xs);
        let iter = elements.xs.map(|b| if b { Bit::One } else { Bit::Zero });
        byte.iter().zip(iter).all(|(i, j)| i == j)
    }

    #[test]
    fn display_() {
        let byte = Byte::from(10);
        println!("byte: {byte}");
    }

    #[test]
    fn debug_() {
        let byte = Byte::from(10);
        println!("byte: {byte:?}");
    }

    #[test]
    fn binary_() {
        let byte = Byte::from(10);
        println!("byte: {byte:b}");
    }

    #[test]
    fn lower_hex_() {
        let byte = Byte::from(10);
        println!("byte: {byte:x}");
    }

    #[test]
    fn upper_hex_() {
        let byte = Byte::from(10);
        println!("byte: {byte:X}");
    }

    #[test]
    fn eq_() {
        assert_eq!(Byte::from(10), Byte::from(10));
        assert_ne!(Byte::from(10), Byte::from(8));
    }

    #[test]
    fn ord_() {
        assert!(Byte::from(8) < Byte::from(10));
    }

    #[test]
    fn u8_from_byte_() {
        let x: u8 = Byte::from(10).into();
        assert_eq!(10, x);
    }

    #[test]
    fn byte_from_u8s_() {
        let xs = [0, 0, 0, 0, 1, 0, 1, 0];
        let byte = Byte::from_iter(xs);
        assert_eq!(byte, 10_u8.into());
    }

    #[test]
    fn byte_from_bools_() {
        let xs = [false, false, false, false, true, false, true, false];
        let byte = Byte::from_iter(xs);
        assert_eq!(byte, 10_u8.into());
    }

    #[test]
    fn is_zero_() {
        assert!(Byte::from(0).is_zero());
        assert!(!Byte::from(10).is_zero());
    }

    #[test]
    fn is_one_() {
        assert!(Byte::from(1).is_one());
        assert!(!Byte::from(10).is_one());
    }

    #[test]
    fn get_bit_() {
        let byte = Byte::from(10);
        assert_eq!(byte.get_bit(0), Bit::Zero);
        assert_eq!(byte.get_bit(4), Bit::One);
    }

    #[test]
    fn set_bit_() {
        let byte = Byte::from(10);
        let byte = byte.set_bit(7);
        assert_eq!(byte, 11.into());
    }

    #[test]
    fn reset_bit_() {
        let byte = Byte::from(10);
        let byte = byte.reset_bit(6);
        assert_eq!(byte, 8.into());
    }

    #[test]
    fn toggle_bit_() {
        let byte = Byte::from(10);
        let byte = byte.toggle_bit(6);
        assert_eq!(byte, 8.into());
    }

    #[test]
    fn byte_iter_() {
        let byte = Byte::from(10);
        let mut iter = byte.iter();

        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn byte_into_iter_() {
        let byte = Byte::from(10);
        let mut iter = byte.into_iter();

        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), None);
    }
}
