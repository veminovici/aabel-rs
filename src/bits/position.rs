use std::fmt::{Debug, Display};

/// Represents a position in the array of bits.
/// 
/// # Examples
/// 
/// ```
/// use aabel_rs::bits::Position;
/// 
/// let pos = Position::from(7);
/// let pos = pos.increment();
/// 
/// assert_eq!(8usize, pos.into());
/// ```
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub(crate) idx: usize,
    pub(crate) bit: u8,
}

const U8SIZE: usize = 8;

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos: usize = (*self).into();
        write!(f, "{pos}")
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.idx, self.bit)
    }
}

impl From<usize> for Position {
    #[inline]
    fn from(idx: usize) -> Self {
        Self {
            idx: idx / U8SIZE,
            bit: (idx % U8SIZE) as u8,
        }
    }
}

impl From<Position> for usize {
    #[inline]
    fn from(pos: Position) -> Self {
        pos.idx * U8SIZE + pos.bit as usize
    }
}

impl Position {
    pub fn increment(self) -> Self {
        let bit = self.bit + 1;
        if bit as usize >= U8SIZE {
            Self {
                idx: self.idx + 1,
                bit: 0,
            }
        } else {
            Self { idx: self.idx, bit }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_() {
        let pos = Position::from(10);
        println!("position: {pos}");
    }

    #[test]
    fn debug_() {
        let pos = Position::from(10);
        println!("position: {pos:?}");
    }

    #[test]
    fn position_from_usize_() {
        let pos = Position::from(10);
        assert_eq!(1, pos.idx);
        assert_eq!(2, pos.bit);
    }

    #[test]
    fn usize_from_position_() {
        let pos = Position::from(10);
        let x: usize = pos.into();
        assert_eq!(x, 10);
    }

    #[test]
    fn eq_() {
        let pos1 = Position::from(10);
        let pos2 = Position::from(8);
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn incr_() {
        let pos = Position::from(6);
        assert_eq!(0, pos.idx);
        assert_eq!(6, pos.bit);

        let pos = pos.increment();
        assert_eq!(0, pos.idx);
        assert_eq!(7, pos.bit);

        let pos = pos.increment();
        assert_eq!(1, pos.idx);
        assert_eq!(0, pos.bit);
    }
}