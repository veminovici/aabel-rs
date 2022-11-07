use super::{Bit, Byte, Position};

/// A vector of bits. Each bit can be accessed and written individually.
pub struct BVec {
    vec: Vec<u8>,
    len: usize,
}

impl BVec {
    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Creates a new instance of the bit-vector with a given length.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::bits::BVec;
    ///
    /// let bvec = BVec::with_length(10);
    /// assert_eq!(10, bvec.len());
    /// ```
    pub fn with_length(len: usize) -> Self {
        let capacity = len / super::U8SIZE + (if len % super::U8SIZE == 0 { 0 } else { 1 });
        let mut vec = Vec::with_capacity(capacity);
        let _x: usize = (0..capacity).inspect(|_| vec.push(0)).sum();

        Self { vec, len }
    }

    /// Returns the bit value from a given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::bits::{Bit, BVec};
    ///
    /// let mut bvec = BVec::with_length(10);
    /// bvec.set_bit(4);
    /// bvec.set_bit(6);
    /// assert_eq!(bvec.get_bit(0), Bit::Zero);
    /// assert_eq!(bvec.get_bit(4), Bit::One);
    /// ```
    pub fn get_bit(&self, bit: usize) -> Bit {
        let pos = Position::from(bit);
        let byte: Byte = self.vec[pos.idx].into();
        byte.get_bit(pos.bit)
    }

    /// Sets the bit value from a given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::bits::{Bit, BVec};
    ///
    /// let mut bvec = BVec::with_length(10);
    /// bvec.set_bit(4);
    /// bvec.set_bit(6);
    /// assert_eq!(bvec.get_bit(0), Bit::Zero);
    /// assert_eq!(bvec.get_bit(4), Bit::One);
    /// ```
    pub fn set_bit(&mut self, bit: usize) {
        let pos = Position::from(bit);
        let byte: Byte = self.vec[pos.idx].into();
        let byte: u8 = byte.set_bit(pos.bit).into();

        let _ = std::mem::replace(&mut self.vec[pos.idx], byte);
    }

    /// Resets the bit value from a given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::bits::{Bit, BVec};
    ///
    /// let mut bvec = BVec::with_length(10);
    /// bvec.set_bit(4);
    /// assert_eq!(bvec.get_bit(4), Bit::One);
    ///
    /// bvec.reset_bit(4);
    /// assert_eq!(bvec.get_bit(4), Bit::Zero);
    /// ```
    pub fn reset_bit(&mut self, bit: usize) {
        let pos = Position::from(bit);
        let byte: Byte = self.vec[pos.idx].into();

        let byte: u8 = byte.reset_bit(pos.bit).into();

        let _ = std::mem::replace(&mut self.vec[pos.idx], byte);
    }

    /// Toggles the bit value from a given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_rs::bits::BVec;
    ///
    /// let mut bvec = BVec::with_length(10);
    /// bvec.toggle_bit(4);
    /// bvec.toggle_bit(6);
    /// ```
    pub fn toggle_bit(&mut self, bit: usize) {
        let pos = Position::from(bit);
        let byte: Byte = self.vec[pos.idx].into();
        let byte: u8 = byte.toggle_bit(pos.bit).into();

        let _ = std::mem::replace(&mut self.vec[pos.idx], byte);
    }
}

impl Extend<Bit> for BVec {
    fn extend<T: IntoIterator<Item = Bit>>(&mut self, iter: T) {
        for bit in iter {
            if self.len == self.vec.capacity() {
                self.vec.extend([0, 0, 0, 0]);
                self.vec.push(0);
                self.vec.push(0);
                self.vec.push(0);
                self.vec.push(0);
            }

            if bit == Bit::One {
                self.set_bit(self.len);
            }

            self.len += 1;
        }
    }
}

pub struct Iter {
    bvec: BVec,
    current: usize,
}

impl Iterator for Iter {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.bvec.len {
            None
        } else {
            let bit = self.bvec.get_bit(self.current);
            self.current += 1;
            Some(bit)
        }
    }
}

impl IntoIterator for BVec {
    type Item = Bit;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            bvec: self,
            current: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_length_() {
        let bvec = BVec::with_length(10);
        assert_eq!(10, bvec.len());
        assert_eq!(2, bvec.vec.capacity());
    }

    #[test]
    fn set_bit_() {
        let mut bvec = BVec::with_length(10);
        bvec.set_bit(4);
        bvec.set_bit(6);
        assert_eq!(10, bvec.vec[0]);
    }

    #[test]
    fn reset_bit_() {
        let mut bvec = BVec::with_length(10);
        bvec.set_bit(4);
        bvec.set_bit(6);
        assert_eq!(10, bvec.vec[0]);

        bvec.reset_bit(6);
        assert_eq!(8, bvec.vec[0]);
    }

    #[test]
    fn get_bit_() {
        let mut bvec = BVec::with_length(10);
        bvec.set_bit(4);
        bvec.set_bit(6);

        assert_eq!(bvec.get_bit(0), Bit::Zero);
        assert_eq!(bvec.get_bit(4), Bit::One);
    }

    #[test]
    fn toggle_bit_() {
        let mut bvec = BVec::with_length(10);
        bvec.toggle_bit(4);
        bvec.toggle_bit(6);

        assert_eq!(10, bvec.vec[0]);
    }

    #[test]
    fn bvec_into_iter_() {
        let mut bvec = BVec::with_length(10);
        bvec.toggle_bit(4);
        bvec.toggle_bit(6);

        let mut iter = bvec.into_iter();

        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn extend_() {
        let mut bvec = BVec::with_length(0);
        assert_eq!(0, bvec.len);
        assert_eq!(0, bvec.vec.capacity());

        let elements = [Bit::One, Bit::Zero, Bit::One, Bit::Zero];
        bvec.extend(elements);

        assert_eq!(4, bvec.len);
        assert_eq!(bvec.get_bit(0), Bit::One);
        assert_eq!(bvec.get_bit(1), Bit::Zero);
        assert_eq!(bvec.get_bit(2), Bit::One);
        assert_eq!(bvec.get_bit(3), Bit::Zero);

        let elements = [Bit::Zero, Bit::Zero, Bit::Zero, Bit::One];
        bvec.extend(elements);

        assert_eq!(8, bvec.len);
        assert_eq!(bvec.get_bit(4), Bit::Zero);
        assert_eq!(bvec.get_bit(5), Bit::Zero);
        assert_eq!(bvec.get_bit(6), Bit::Zero);
        assert_eq!(bvec.get_bit(7), Bit::One);

        let elements = [Bit::One, Bit::Zero, Bit::One, Bit::Zero];
        bvec.extend(elements);

        assert_eq!(12, bvec.len);
        assert_eq!(bvec.get_bit(8), Bit::One);
        assert_eq!(bvec.get_bit(9), Bit::Zero);
        assert_eq!(bvec.get_bit(10), Bit::One);
        assert_eq!(bvec.get_bit(11), Bit::Zero);
    }
}
