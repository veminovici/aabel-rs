use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Bit {
    Zero = 0x0,
    One = 0x1,
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
        }
    }
}

impl Debug for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "B0"),
            Self::One => write!(f, "B1"),
        }
    }
}

impl From<u8> for Bit {
    #[inline]
    fn from(value: u8) -> Self {
        if value == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl From<Bit> for u8 {
    #[inline]
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl From<bool> for Bit {
    #[inline]
    fn from(b: bool) -> Self {
        if b {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl BitAnd<Bit> for Bit {
    type Output = Bit;

    #[inline]
    fn bitand(self, rhs: Bit) -> Self::Output {
        if self == Bit::Zero {
            Bit::Zero
        } else if rhs == Bit::Zero {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl BitAnd<u8> for Bit {
    type Output = Bit;

    #[inline]
    fn bitand(self, rhs: u8) -> Self::Output {
        if self == Bit::Zero {
            Bit::Zero
        } else if rhs == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl BitOr<Bit> for Bit {
    type Output = Bit;

    #[inline]
    fn bitor(self, rhs: Bit) -> Self::Output {
        if self == Bit::One {
            Bit::One
        } else if rhs == Bit::One {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl BitOr<u8> for Bit {
    type Output = Bit;

    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        if self == Bit::One {
            Bit::One
        } else if rhs != 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_() {
        println!("zro: {}", Bit::Zero);
        println!("one: {}", Bit::One);
    }

    #[test]
    fn debug_() {
        println!("zro: {:?}", Bit::Zero);
        println!("one: {:?}", Bit::One);
    }

    #[test]
    fn eq_() {
        assert_ne!(Bit::One, Bit::Zero);
    }

    #[test]
    fn bit_from_u8_() {
        let bit = Bit::from(0_u8);
        assert_eq!(Bit::Zero, bit);

        let bit = Bit::from(10_u8);
        assert_eq!(Bit::One, bit);
    }

    #[test]
    fn u8_from_bit_() {
        let x: u8 = Bit::Zero.into();
        assert_eq!(0, x);

        let x: u8 = Bit::One.into();
        assert_eq!(1, x);
    }

    #[test]
    fn bit_from_bool_() {
        let bit = Bit::from(false);
        assert_eq!(Bit::Zero, bit);

        let bit = Bit::from(true);
        assert_eq!(Bit::One, bit);
    }

    #[test]
    fn bool_from_bit_() {
        let b: bool = Bit::Zero.into();
        assert_eq!(false, b);

        let b: bool = Bit::One.into();
        assert_eq!(true, b);
    }

    #[test]
    fn bit_or_bit_() {
        let bit = Bit::One | Bit::Zero;
        assert_eq!(bit, Bit::One);

        let bit = Bit::Zero | Bit::One;
        assert_eq!(bit, Bit::One);

        let bit = Bit::Zero | Bit::Zero;
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn bit_or_u8_() {
        let bit = Bit::One | 0_u8;
        assert_eq!(bit, Bit::One);

        let bit = Bit::Zero | 1_u8;
        assert_eq!(bit, Bit::One);

        let bit = Bit::Zero | 0_u8;
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn bit_and_bit_() {
        let bit = Bit::Zero & Bit::One;
        assert_eq!(bit, Bit::Zero);

        let bit = Bit::One & Bit::Zero;
        assert_eq!(bit, Bit::Zero);

        let bit = Bit::One & Bit::One;
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn bit_and_u8_() {
        let bit = Bit::Zero & 1_u8;
        assert_eq!(bit, Bit::Zero);

        let bit = Bit::One & 0_u8;
        assert_eq!(bit, Bit::Zero);

        let bit = Bit::One & 1_u8;
        assert_eq!(bit, Bit::One);
    }
}