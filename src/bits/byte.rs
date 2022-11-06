use std::fmt::{Binary, Debug, Display, LowerHex, UpperHex};

use super::Bit;

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
        let mask = 1 << (7 - bit);
        if self.0 & mask == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }

    #[inline]
    pub fn set_bit(self, bit: u8) -> Self {
        let mask = 1 << (7 - bit);
        Self(self.0 | mask)
    }

    #[inline]
    pub fn reset_bit(self, bit: u8) -> Self {
        let mask = !(1 << (7 - bit));
        Self(self.0 & mask)
    }

    #[inline]
    pub fn toggle_bit(self, bit: u8) -> Self {
        let mask = 1 << (7 - bit);
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
        let bit = Byte::from_iter(xs);
        assert_eq!(bit, 10_u8.into());
    }

    #[test]
    fn byte_from_bools_() {
        let xs = [false, false, false, false, true, false, true, false];
        let bit = Byte::from_iter(xs);
        assert_eq!(bit, 10_u8.into());
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
